use yew::prelude::*;

pub enum Msg {}

/// # BussinesAssets
///
/// Add a svg BussinesIcon
pub struct BussinesAssets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of BussinesIcons
    pub icon: BussinesIcon,
    /// Size of the BussinesIcon
    #[prop_or(("24".to_string(),"24".to_string()))]
    pub size: (String, String),
    /// Defines the position and dimension of the BussinesIcon
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

impl Component for BussinesAssets {
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
            pub enum BussinesIcon {
        DollarSign,
Target,
BarChart,
CreditCard,
TrendingDown,
Percent,
BarChart2,
PieChart,
TrendingUp,
Award,

}

                fn get_icon(
                    icon: BussinesIcon,
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    match icon {
            BussinesIcon::DollarSign => get_dollarsign(size, view_box, fill, class_name, id),
BussinesIcon::Target => get_target(size, view_box, fill, class_name, id),
BussinesIcon::BarChart => get_barchart(size, view_box, fill, class_name, id),
BussinesIcon::CreditCard => get_creditcard(size, view_box, fill, class_name, id),
BussinesIcon::TrendingDown => get_trendingdown(size, view_box, fill, class_name, id),
BussinesIcon::Percent => get_percent(size, view_box, fill, class_name, id),
BussinesIcon::BarChart2 => get_barchart2(size, view_box, fill, class_name, id),
BussinesIcon::PieChart => get_piechart(size, view_box, fill, class_name, id),
BussinesIcon::TrendingUp => get_trendingup(size, view_box, fill, class_name, id),
BussinesIcon::Award => get_award(size, view_box, fill, class_name, id),

}
}

                fn get_dollarsign(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-dollar-sign"><line x1="12" y1="1" x2="12" y2="23"></line><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path></svg>
                    }
                }
            
                fn get_target(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="6"></circle><circle cx="12" cy="12" r="2"></circle></svg>
                    }
                }
            
                fn get_barchart(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-bar-chart"><line x1="12" y1="20" x2="12" y2="10"></line><line x1="18" y1="20" x2="18" y2="4"></line><line x1="6" y1="20" x2="6" y2="16"></line></svg>
                    }
                }
            
                fn get_creditcard(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-credit-card"><rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect><line x1="1" y1="10" x2="23" y2="10"></line></svg>
                    }
                }
            
                fn get_trendingdown(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trending-down"><polyline points="23 18 13.5 8.5 8.5 13.5 1 6"></polyline><polyline points="17 18 23 18 23 12"></polyline></svg>
                    }
                }
            
                fn get_percent(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><line x1="19" y1="5" x2="5" y2="19"></line><circle cx="6.5" cy="6.5" r="2.5"></circle><circle cx="17.5" cy="17.5" r="2.5"></circle></svg>
                    }
                }
            
                fn get_barchart2(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-bar-chart-2"><line x1="18" y1="20" x2="18" y2="10"></line><line x1="12" y1="20" x2="12" y2="4"></line><line x1="6" y1="20" x2="6" y2="14"></line></svg>
                    }
                }
            
                fn get_piechart(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-pie-chart"><path d="M21.21 15.89A10 10 0 1 1 8 2.83"></path><path d="M22 12A10 10 0 0 0 12 2v10z"></path></svg>
                    }
                }
            
                fn get_trendingup(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trending-up"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline><polyline points="17 6 23 6 23 12"></polyline></svg>
                    }
                }
            
                fn get_award(
                    size: (String, String),
                    view_box: (String, String, String, String),
                    fill: String,
                    class_name: String,
                    id: String,
                ) -> Html {
                    html!{
                        <svg xmlns="http://www.w3.org/2000/svg" witdh=size.0 height=size.1 viewBox=format!("{} {} {} {}",
                            view_box.0,
                            view_box.1,
                            view_box.2,
                            view_box.3,
                        ) fill=fill id=id stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class_name><circle cx="12" cy="8" r="7"></circle><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"></polyline></svg>
                    }
                }
            