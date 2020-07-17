use yew::prelude::*;

pub enum Msg {}

/// # MultimediaAssets
///
/// Add a svg MultimediaIcon
///
/// ## Feature
/// multimedia_assets
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_assets::{MultimediaAssets, MultimediaIcon};
///
/// pub struct App;

/// impl Component for App {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
///         App {}
///     }
///
///     fn update(&mut self, _: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         <MultimediaAssets
///             icon = MultimediaIcon::Video
///             fill = "#fff"
///             size = ("30".to_string(),"30".to_string())
///         />
///     }
/// }
/// ```
pub struct MultimediaAssets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of MultimediaIcons
    pub icon: MultimediaIcon,
    /// Size of the MultimediaIcon
    #[prop_or(("24".to_string(),"24".to_string()))]
    pub size: (String, String),
    /// Defines the position and dimension of the MultimediaIcon
    #[prop_or(("0".to_string(),"0".to_string(),"24".to_string(),"24".to_string()))]
    pub view_box: (String, String, String, String),
    /// Fill the color of the asset
    #[prop_or("none".to_string())]
    pub fill: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for MultimediaAssets {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        get_icon(
            self.props.icon.clone(),
            self.props.size.clone(),
            self.props.view_box.clone(),
            self.props.fill.clone(),
            self.props.class_name.clone(),
            self.props.id.clone(),
        )
    }
}

#[derive(Clone)]
pub enum MultimediaIcon {
    Video,
    Film,
    Radio,
    Headphones,
    Music,
    VideoOff,
    Youtube,
    Airplay,
}

fn get_icon(
    icon: MultimediaIcon,
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    match icon {
        MultimediaIcon::Video => get_video(size, view_box, fill, class_name, id),
        MultimediaIcon::Film => get_film(size, view_box, fill, class_name, id),
        MultimediaIcon::Radio => get_radio(size, view_box, fill, class_name, id),
        MultimediaIcon::Headphones => get_headphones(size, view_box, fill, class_name, id),
        MultimediaIcon::Music => get_music(size, view_box, fill, class_name, id),
        MultimediaIcon::VideoOff => get_videooff(size, view_box, fill, class_name, id),
        MultimediaIcon::Youtube => get_youtube(size, view_box, fill, class_name, id),
        MultimediaIcon::Airplay => get_airplay(size, view_box, fill, class_name, id),
    }
}

fn get_video(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polygon points="23 7 16 12 23 17 23 7"></polygon><rect x="1" y="5" width="15" height="14" rx="2" ry="2"></rect></svg>
    }
}

fn get_film(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18"></rect><line x1="7" y1="2" x2="7" y2="22"></line><line x1="17" y1="2" x2="17" y2="22"></line><line x1="2" y1="12" x2="22" y2="12"></line><line x1="2" y1="7" x2="7" y2="7"></line><line x1="2" y1="17" x2="7" y2="17"></line><line x1="17" y1="17" x2="22" y2="17"></line><line x1="17" y1="7" x2="22" y2="7"></line></svg>
    }
}

fn get_radio(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="2"></circle><path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14"></path></svg>
    }
}

fn get_headphones(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M3 18v-6a9 9 0 0 1 18 0v6"></path><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"></path></svg>
    }
}

fn get_music(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M9 18V5l12-2v13"></path><circle cx="6" cy="18" r="3"></circle><circle cx="18" cy="16" r="3"></circle></svg>
    }
}

fn get_videooff(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
    }
}

fn get_youtube(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path><polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon></svg>
    }
}

fn get_airplay(
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
            view_box.0,
            view_box.1,
            view_box.2,
            view_box.3,
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"></path><polygon points="12 15 17 21 7 21 12 15"></polygon></svg>
    }
}
