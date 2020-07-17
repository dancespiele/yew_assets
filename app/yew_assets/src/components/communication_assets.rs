use yew::prelude::*;

pub enum Msg {}

/// # CommunicationAssets
///
/// Add a svg CommunicationIcon
///
/// ## Feature
/// communication_assets
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_assets::{CommunicationAssetss, CommunicationIcon};
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
///         <CommunicationAssets
///             icon = CommunicationIcon::Users
///             fill = "#fff"
///             size = ("30".to_string(),"30".to_string())
///         />
///     }
/// }
/// ```
pub struct CommunicationAssets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of CommunicationIcons
    pub icon: CommunicationIcon,
    /// Size of the CommunicationIcon
    #[prop_or(("24".to_string(),"24".to_string()))]
    pub size: (String, String),
    /// Defines the position and dimension of the CommunicationIcon
    #[prop_or(("0".to_string(),"0".to_string(),"24".to_string(),"24".to_string()))]
    pub view_box: (String, String, String, String),
    /// fill the color of the asset
    #[prop_or("none".to_string())]
    pub fill: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for CommunicationAssets {
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
pub enum CommunicationIcon {
    Users,
    User,
    UserPlus,
    Frown,
    PhoneMissed,
    PhoneCall,
    UserX,
    PhoneOff,
    Star,
    UserCheck,
    Meh,
    PhoneOutgoing,
    Smile,
    Bluetooth,
    UserMinus,
    Voicemail,
    PhoneIncoming,
    Phone,
    WifiOff,
    Mail,
    MessageCircle,
    PhoneForwarded,
    Heart,
    MessageSquare,
    Wifi,
}

fn get_icon(
    icon: CommunicationIcon,
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    match icon {
        CommunicationIcon::Users => get_users(size, view_box, fill, class_name, id),
        CommunicationIcon::User => get_user(size, view_box, fill, class_name, id),
        CommunicationIcon::UserPlus => get_userplus(size, view_box, fill, class_name, id),
        CommunicationIcon::Frown => get_frown(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneMissed => get_phonemissed(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneCall => get_phonecall(size, view_box, fill, class_name, id),
        CommunicationIcon::UserX => get_userx(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneOff => get_phoneoff(size, view_box, fill, class_name, id),
        CommunicationIcon::Star => get_star(size, view_box, fill, class_name, id),
        CommunicationIcon::UserCheck => get_usercheck(size, view_box, fill, class_name, id),
        CommunicationIcon::Meh => get_meh(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneOutgoing => get_phoneoutgoing(size, view_box, fill, class_name, id),
        CommunicationIcon::Smile => get_smile(size, view_box, fill, class_name, id),
        CommunicationIcon::Bluetooth => get_bluetooth(size, view_box, fill, class_name, id),
        CommunicationIcon::UserMinus => get_userminus(size, view_box, fill, class_name, id),
        CommunicationIcon::Voicemail => get_voicemail(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneIncoming => get_phoneincoming(size, view_box, fill, class_name, id),
        CommunicationIcon::Phone => get_phone(size, view_box, fill, class_name, id),
        CommunicationIcon::WifiOff => get_wifioff(size, view_box, fill, class_name, id),
        CommunicationIcon::Mail => get_mail(size, view_box, fill, class_name, id),
        CommunicationIcon::MessageCircle => get_messagecircle(size, view_box, fill, class_name, id),
        CommunicationIcon::PhoneForwarded => {
            get_phoneforwarded(size, view_box, fill, class_name, id)
        }
        CommunicationIcon::Heart => get_heart(size, view_box, fill, class_name, id),
        CommunicationIcon::MessageSquare => get_messagesquare(size, view_box, fill, class_name, id),
        CommunicationIcon::Wifi => get_wifi(size, view_box, fill, class_name, id),
    }
}

fn get_users(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>
    }
}

fn get_user(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>
    }
}

fn get_userplus(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="20" y1="8" x2="20" y2="14"></line><line x1="23" y1="11" x2="17" y2="11"></line></svg>
    }
}

fn get_frown(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><path d="M16 16s-1.5-2-4-2-4 2-4 2"></path><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line></svg>
    }
}

fn get_phonemissed(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="23" y1="1" x2="17" y2="7"></line><line x1="17" y1="1" x2="23" y2="7"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_phonecall(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_userx(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="18" y1="8" x2="23" y2="13"></line><line x1="23" y1="8" x2="18" y2="13"></line></svg>
    }
}

fn get_phoneoff(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91"></path><line x1="23" y1="1" x2="1" y2="23"></line></svg>
    }
}

fn get_star(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
    }
}

fn get_usercheck(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><polyline points="17 11 19 13 23 9"></polyline></svg>
    }
}

fn get_meh(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><line x1="8" y1="15" x2="16" y2="15"></line><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line></svg>
    }
}

fn get_phoneoutgoing(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="23 7 23 1 17 1"></polyline><line x1="16" y1="8" x2="23" y2="1"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_smile(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><path d="M8 14s1.5 2 4 2 4-2 4-2"></path><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line></svg>
    }
}

fn get_bluetooth(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5"></polyline></svg>
    }
}

fn get_userminus(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="23" y1="11" x2="17" y2="11"></line></svg>
    }
}

fn get_voicemail(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="5.5" cy="11.5" r="4.5"></circle><circle cx="18.5" cy="11.5" r="4.5"></circle><line x1="5.5" y1="16" x2="18.5" y2="16"></line></svg>
    }
}

fn get_phoneincoming(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="16 2 16 8 22 8"></polyline><line x1="23" y1="1" x2="16" y2="8"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_phone(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_wifioff(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="1" y1="1" x2="23" y2="23"></line><path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"></path><path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"></path><path d="M10.71 5.05A16 16 0 0 1 22.58 9"></path><path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"></path><path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path><line x1="12" y1="20" x2="12.01" y2="20"></line></svg>
    }
}

fn get_mail(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path><polyline points="22,6 12,13 2,6"></polyline></svg>
    }
}

fn get_messagecircle(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path></svg>
    }
}

fn get_phoneforwarded(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="19 1 23 5 19 9"></polyline><line x1="15" y1="5" x2="23" y2="5"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
    }
}

fn get_heart(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path></svg>
    }
}

fn get_messagesquare(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg>
    }
}

fn get_wifi(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M5 12.55a11 11 0 0 1 14.08 0"></path><path d="M1.42 9a16 16 0 0 1 21.16 0"></path><path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path><line x1="12" y1="20" x2="12.01" y2="20"></line></svg>
    }
}
