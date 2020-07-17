use yew::prelude::*;

pub enum Msg {}

/// # EditingAssets
///
/// Add a svg EditingIcon
///
/// ## Feature
/// editing_assets
///
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_assets::{EditingAssets, EditingIcon};
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
///         <EditingAssets
///             icon = EditingIcon::XCircle
///             fill = "#fff"
///             size = ("30".to_string(),"30".to_string())
///         />
///     }
/// }
/// ```
pub struct EditingAssets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of EditingIcons
    pub icon: EditingIcon,
    /// Size of the EditingIcon
    #[prop_or(("24".to_string(),"24".to_string()))]
    pub size: (String, String),
    /// Defines the position and dimension of the EditingIcon
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

impl Component for EditingAssets {
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
pub enum EditingIcon {
    XCircle,
    Crop,
    Type,
    Minimize2,
    CheckCircle,
    ZapOff,
    Trash2,
    MinusSquare,
    AlignRight,
    Bold,
    X,
    Italic,
    XSquare,
    Underline,
    PlusSquare,
    Minus,
    Scissors,
    ZoomIn,
    Edit2,
    Maximize2,
    Edit,
    AlignJustify,
    List,
    Delete,
    ZoomOut,
    XOctagon,
    Minimize,
    Save,
    AlignLeft,
    Zap,
    MinusCircle,
    CheckSquare,
    AlignCenter,
    Move,
    Copy,
    Trash,
    Maximize,
    Plus,
    Check,
    PlusCircle,
    Edit3,
}

fn get_icon(
    icon: EditingIcon,
    size: (String, String),
    view_box: (String, String, String, String),
    fill: String,
    class_name: String,
    id: String,
) -> Html {
    match icon {
        EditingIcon::XCircle => get_xcircle(size, view_box, fill, class_name, id),
        EditingIcon::Crop => get_crop(size, view_box, fill, class_name, id),
        EditingIcon::Type => get_type(size, view_box, fill, class_name, id),
        EditingIcon::Minimize2 => get_minimize2(size, view_box, fill, class_name, id),
        EditingIcon::CheckCircle => get_checkcircle(size, view_box, fill, class_name, id),
        EditingIcon::ZapOff => get_zapoff(size, view_box, fill, class_name, id),
        EditingIcon::Trash2 => get_trash2(size, view_box, fill, class_name, id),
        EditingIcon::MinusSquare => get_minussquare(size, view_box, fill, class_name, id),
        EditingIcon::AlignRight => get_alignright(size, view_box, fill, class_name, id),
        EditingIcon::Bold => get_bold(size, view_box, fill, class_name, id),
        EditingIcon::X => get_x(size, view_box, fill, class_name, id),
        EditingIcon::Italic => get_italic(size, view_box, fill, class_name, id),
        EditingIcon::XSquare => get_xsquare(size, view_box, fill, class_name, id),
        EditingIcon::Underline => get_underline(size, view_box, fill, class_name, id),
        EditingIcon::PlusSquare => get_plussquare(size, view_box, fill, class_name, id),
        EditingIcon::Minus => get_minus(size, view_box, fill, class_name, id),
        EditingIcon::Scissors => get_scissors(size, view_box, fill, class_name, id),
        EditingIcon::ZoomIn => get_zoomin(size, view_box, fill, class_name, id),
        EditingIcon::Edit2 => get_edit2(size, view_box, fill, class_name, id),
        EditingIcon::Maximize2 => get_maximize2(size, view_box, fill, class_name, id),
        EditingIcon::Edit => get_edit(size, view_box, fill, class_name, id),
        EditingIcon::AlignJustify => get_alignjustify(size, view_box, fill, class_name, id),
        EditingIcon::List => get_list(size, view_box, fill, class_name, id),
        EditingIcon::Delete => get_delete(size, view_box, fill, class_name, id),
        EditingIcon::ZoomOut => get_zoomout(size, view_box, fill, class_name, id),
        EditingIcon::XOctagon => get_xoctagon(size, view_box, fill, class_name, id),
        EditingIcon::Minimize => get_minimize(size, view_box, fill, class_name, id),
        EditingIcon::Save => get_save(size, view_box, fill, class_name, id),
        EditingIcon::AlignLeft => get_alignleft(size, view_box, fill, class_name, id),
        EditingIcon::Zap => get_zap(size, view_box, fill, class_name, id),
        EditingIcon::MinusCircle => get_minuscircle(size, view_box, fill, class_name, id),
        EditingIcon::CheckSquare => get_checksquare(size, view_box, fill, class_name, id),
        EditingIcon::AlignCenter => get_aligncenter(size, view_box, fill, class_name, id),
        EditingIcon::Move => get_move(size, view_box, fill, class_name, id),
        EditingIcon::Copy => get_copy(size, view_box, fill, class_name, id),
        EditingIcon::Trash => get_trash(size, view_box, fill, class_name, id),
        EditingIcon::Maximize => get_maximize(size, view_box, fill, class_name, id),
        EditingIcon::Plus => get_plus(size, view_box, fill, class_name, id),
        EditingIcon::Check => get_check(size, view_box, fill, class_name, id),
        EditingIcon::PlusCircle => get_pluscircle(size, view_box, fill, class_name, id),
        EditingIcon::Edit3 => get_edit3(size, view_box, fill, class_name, id),
    }
}

fn get_xcircle(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
    }
}

fn get_crop(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M6.13 1L6 16a2 2 0 0 0 2 2h15"></path><path d="M1 6.13L16 6a2 2 0 0 1 2 2v15"></path></svg>
    }
}

fn get_type(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="4 7 4 4 20 4 20 7"></polyline><line x1="9" y1="20" x2="15" y2="20"></line><line x1="12" y1="4" x2="12" y2="20"></line></svg>
    }
}

fn get_minimize2(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="4 14 10 14 10 20"></polyline><polyline points="20 10 14 10 14 4"></polyline><line x1="14" y1="10" x2="21" y2="3"></line><line x1="3" y1="21" x2="10" y2="14"></line></svg>
    }
}

fn get_checkcircle(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
    }
}

fn get_zapoff(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="12.41 6.75 13 2 10.57 4.92"></polyline><polyline points="18.57 12.91 21 10 15.66 10"></polyline><polyline points="8 8 3 14 12 14 11 22 16 16"></polyline><line x1="1" y1="1" x2="23" y2="23"></line></svg>
    }
}

fn get_trash2(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
    }
}

fn get_minussquare(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="8" y1="12" x2="16" y2="12"></line></svg>
    }
}

fn get_alignright(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="21" y1="10" x2="7" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="21" y1="18" x2="7" y2="18"></line></svg>
    }
}

fn get_bold(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path></svg>
    }
}

fn get_x(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
    }
}

fn get_italic(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="19" y1="4" x2="10" y2="4"></line><line x1="14" y1="20" x2="5" y2="20"></line><line x1="15" y1="4" x2="9" y2="20"></line></svg>
    }
}

fn get_xsquare(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="9" x2="15" y2="15"></line><line x1="15" y1="9" x2="9" y2="15"></line></svg>
    }
}

fn get_underline(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3"></path><line x1="4" y1="21" x2="20" y2="21"></line></svg>
    }
}

fn get_plussquare(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg>
    }
}

fn get_minus(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="5" y1="12" x2="19" y2="12"></line></svg>
    }
}

fn get_scissors(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="6" cy="6" r="3"></circle><circle cx="6" cy="18" r="3"></circle><line x1="20" y1="4" x2="8.12" y2="15.88"></line><line x1="14.47" y1="14.48" x2="20" y2="20"></line><line x1="8.12" y1="8.12" x2="12" y2="12"></line></svg>
    }
}

fn get_zoomin(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="11" y1="8" x2="11" y2="14"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
    }
}

fn get_edit2(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path></svg>
    }
}

fn get_maximize2(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="15 3 21 3 21 9"></polyline><polyline points="9 21 3 21 3 15"></polyline><line x1="21" y1="3" x2="14" y2="10"></line><line x1="3" y1="21" x2="10" y2="14"></line></svg>
    }
}

fn get_edit(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
    }
}

fn get_alignjustify(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="21" y1="10" x2="3" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="21" y1="18" x2="3" y2="18"></line></svg>
    }
}

fn get_list(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line></svg>
    }
}

fn get_delete(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z"></path><line x1="18" y1="9" x2="12" y2="15"></line><line x1="12" y1="9" x2="18" y2="15"></line></svg>
    }
}

fn get_zoomout(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
    }
}

fn get_xoctagon(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
    }
}

fn get_minimize(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3"></path></svg>
    }
}

fn get_save(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path><polyline points="17 21 17 13 7 13 7 21"></polyline><polyline points="7 3 7 8 15 8"></polyline></svg>
    }
}

fn get_alignleft(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="17" y1="10" x2="3" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="17" y1="18" x2="3" y2="18"></line></svg>
    }
}

fn get_zap(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon></svg>
    }
}

fn get_minuscircle(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><line x1="8" y1="12" x2="16" y2="12"></line></svg>
    }
}

fn get_checksquare(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="9 11 12 14 22 4"></polyline><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path></svg>
    }
}

fn get_aligncenter(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="18" y1="10" x2="6" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="18" y1="18" x2="6" y2="18"></line></svg>
    }
}

fn get_move(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="5 9 2 12 5 15"></polyline><polyline points="9 5 12 2 15 5"></polyline><polyline points="15 19 12 22 9 19"></polyline><polyline points="19 9 22 12 19 15"></polyline><line x1="2" y1="12" x2="22" y2="12"></line><line x1="12" y1="2" x2="12" y2="22"></line></svg>
    }
}

fn get_copy(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
    }
}

fn get_trash(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
    }
}

fn get_maximize(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
    }
}

fn get_plus(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
    }
}

fn get_check(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><polyline points="20 6 9 17 4 12"></polyline></svg>
    }
}

fn get_pluscircle(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg>
    }
}

fn get_edit3(
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
        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><path d="M12 20h9"></path><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path></svg>
    }
}
