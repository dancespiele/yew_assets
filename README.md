# Yew Assets
Assets Icon components for yew. The svgs are created by [feather community](https://feathericons.com) and all of them have the most permissive license (MIT)

## How to use:

Include in cargo.toml with the features which will be used in the project:
```toml
yew_assets = {version="0.1", features=["ux_assets", "editing_assets", "social_assets"]}
```

### Features
browser_assets, business_assets, communication_assets, controller_assets, dev_assets, device_assets, editing_assets, env_assets, file_assets, info_assets, multimedia_assets, nav_assets, object_assets, social_assets, ux_assets

### Example of one of the Assets Icon component
```rust
use yew::prelude::*;
use yew_assets::{UxAssets, UxIcon};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        <UxAssets
            icon = UxIcon::ShieldOff
            fill = "#fff"
            size = ("30".to_string(),"30".to_string())
        />
    }
}
```

## How run documentation page

1. Clone the repository:
`git clone git@github.com:spielrs/yew_assets.git`
2. Run the project:
`RUST_LOG=yew_assets cargo=trace cargo run`
3. Open the browser in [http://127.0.0.1:8080](http://127.0.0.1:8080)

## How to contributed
For a new svg icon please open a PR in [feather github repository](https://github.com/feathericons/feather) and after it is approved for them, create a new issue
in this repository then soon the component will be generated.
For fixes please open directly a pull request.

Yew Style is [MIT](LICENSE-MIT.md) and [Apache-2.0](LICENSE-APACHE.md) licensed