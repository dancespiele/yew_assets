extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use futures::{
    future::{abortable, join, AbortHandle},
    FutureExt, SinkExt, StreamExt,
};
use hyper::server::Server;
use hyper::service::make_service_fn;
use spielrs_diff::dir_diff;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use std::{convert::Infallible, process::Stdio};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::delay_for;
use warp::{
    ws::{Message, WebSocket},
    Error, Filter,
};

#[tokio::main(core_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    dotenv().ok();

    let server_address: SocketAddr = get_server_address().parse().unwrap();
    let wasm_path = get_wasm_path();

    build_wasm().await.unwrap();

    let websocket_routes = warp::ws().map(|ws: warp::ws::Ws| ws.on_upgrade(connect));

    let websocket_service = warp::service(websocket_routes);

    let make_websocket_svc = make_service_fn(|_: _| {
        let websocket_service = websocket_service;
        async move { Ok::<_, Infallible>(websocket_service) }
    });

    let websocket_srv = Server::bind(&([127, 0, 0, 1], 3000).into()).serve(make_websocket_svc);

    let index = warp::get().and(warp::fs::file(format!("{}/index.html", wasm_path)));
    let core = warp::path("pkg").and(warp::fs::dir(format!("{}/pkg", wasm_path)));
    let static_files = warp::path("static").and(warp::fs::dir(format!("{}/static", wasm_path)));
    let node_modules =
        warp::path("node_modules").and(warp::fs::dir(format!("{}/node_modules", wasm_path)));
    let routes = core.or(node_modules.or(static_files.or(index)));
    let svc = warp::service(routes);
    let make_svc = make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let srv = Server::bind(&server_address).serve(make_svc);
    info!("stat server in address {}", server_address);

    let _ret = join(websocket_srv, srv).await;

    Ok(())
}

fn get_env_build() -> String {
    if let Ok(env_build) = env::var("ENV_BUILD") {
        if env_build == "production" {
            "--release".to_string()
        } else {
            "--dev".to_string()
        }
    } else {
        "--dev".to_string()
    }
}

fn get_wasm_path() -> String {
    if let Ok(wasm_path) = env::var("WASM_PATH") {
        wasm_path
    } else {
        "app".to_string()
    }
}

fn get_server_address() -> String {
    if let Ok(server_address) = env::var("SERVER_ADRESS") {
        server_address
    } else {
        "127.0.0.1:8080".to_string()
    }
}

async fn build_wasm() -> Result<(), Box<dyn std::error::Error>> {
    let env_build = get_env_build();
    let wasm_path = get_wasm_path();
    let mut cmd = Command::new("wasm-pack");
    let cmd = cmd
        .arg("build")
        .arg(env_build)
        .args(&["--target", "web"])
        .current_dir(wasm_path);

    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        info!("{}", line);
    }
    Ok(())
}

async fn connect(ws: WebSocket) {
    let (mut browser_ws_tx, mut browser_ws_rx) = ws.split();
    browser_ws_tx.send(Message::text("reload")).await.unwrap();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::task::spawn(rx.forward(browser_ws_tx).map(|result| {
        if let Err(e) = result {
            warn!("{}", e);
        }
    }));

    tokio::task::spawn(watch(tx.clone()));

    while let Some(result) = browser_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                warn!("websocket {}", e);
                break;
            }
        };

        match msg.to_str() {
            Ok(m) => info!("{}", m),
            Err(_e) => (),
        }
    }
}

async fn watch(tx: mpsc::UnboundedSender<Result<Message, Error>>) {
    let wasm_path = get_wasm_path();
    let mut process: Vec<AbortHandle> = vec![];
    loop {
        let tx = tx.clone();
        let wasm_path = wasm_path.clone();
        match fs::read_dir(".diff").await {
            Ok(_file) => (),
            Err(_e) => {
                Command::new("cp")
                    .arg("-R")
                    .args(&[format!("{}/src", wasm_path), ".diff".to_string()])
                    .output()
                    .await
                    .unwrap();
            }
        };

        let diff = dir_diff(format!("{}/src", wasm_path), ".diff".to_string()).await;

        if diff {
            for p in process.clone() {
                p.abort();
                process.pop();
            }

            let tx = tx.clone();
            let (fut, handle) = abortable(async move {
                info!("sources change");
                build_wasm().await.unwrap();
                tx.send(Ok(Message::text("reload"))).unwrap();
                Command::new("rm")
                    .arg("-R")
                    .arg(".diff")
                    .output()
                    .await
                    .unwrap();
            });

            process.push(handle);

            fut.await.unwrap();
        }
        delay_for(Duration::from_millis(2000)).await;
    }
}
