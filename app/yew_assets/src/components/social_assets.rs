use yew::prelude::*;

pub enum Msg {}

/// # SocialAssets
///
/// Add a svg SocialIcon
///
/// ## Feature
/// social_assets
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_assets::{SocialAssets, SocialIcon};
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
///         <SocialAssets
///             icon = SocialIcon::ThumbsUp
///             fill = "#fff"
///             size = ("30".to_string(),"30".to_string())
///         />
///     }
/// }
/// ```
pub struct SocialAssets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of SocialIcons
    pub icon: SocialIcon,
    /// Size of the SocialIcon
    #[prop_or(("24".to_string(),"24".to_string()))]
    pub size: (String, String),
    /// Defines the position and dimension of the SocialIcon
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

impl Component for SocialAssets {
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
pub enum SocialIcon {
    ThumbsUp,
    Twitch,
    Instagram,
    Slack,
    Hash,
    Linkedin,
    Twitter,
    ThumbsDown,
    Facebook,
    Rss,
    Slash,
}

fn get_icon(
    icon: SocialIcon,
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    match icon {
        SocialIcon::ThumbsUp => get_thumbsup(size, view_box, fill, class_name, id),
        SocialIcon::Twitch => get_twitch(size, view_box, fill, class_name, id),
        SocialIcon::Instagram => get_instagram(size, view_box, fill, class_name, id),
        SocialIcon::Slack => get_slack(size, view_box, fill, class_name, id),
        SocialIcon::Hash => get_hash(size, view_box, fill, class_name, id),
        SocialIcon::Linkedin => get_linkedin(size, view_box, fill, class_name, id),
        SocialIcon::Twitter => get_twitter(size, view_box, fill, class_name, id),
        SocialIcon::ThumbsDown => get_thumbsdown(size, view_box, fill, class_name, id),
        SocialIcon::Facebook => get_facebook(size, view_box, fill, class_name, id),
        SocialIcon::Rss => get_rss(size, view_box, fill, class_name, id),
        SocialIcon::Slash => get_slash(size, view_box, fill, class_name, id),
    }
}

fn get_thumbsup(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3"></path></svg>
    }
}

fn get_twitch(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7"></path></svg>
    }
}

fn get_instagram(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line></svg>
    }
}

fn get_slack(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z"></path><path d="M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"></path><path d="M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z"></path><path d="M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z"></path><path d="M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z"></path><path d="M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z"></path><path d="M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z"></path><path d="M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z"></path></svg>
    }
}

fn get_hash(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="4" y1="9" x2="20" y2="9"></line><line x1="4" y1="15" x2="20" y2="15"></line><line x1="10" y1="3" x2="8" y2="21"></line><line x1="16" y1="3" x2="14" y2="21"></line></svg>
    }
}

fn get_linkedin(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect x="2" y="9" width="4" height="12"></rect><circle cx="4" cy="4" r="2"></circle></svg>
    }
}

fn get_twitter(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"></path></svg>
    }
}

fn get_thumbsdown(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17"></path></svg>
    }
}

fn get_facebook(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path></svg>
    }
}

fn get_rss(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M4 11a9 9 0 0 1 9 9"></path><path d="M4 4a16 16 0 0 1 16 16"></path><circle cx="5" cy="19" r="1"></circle></svg>
    }
}

fn get_slash(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line></svg>
    }
}
