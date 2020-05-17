use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Wasm Warp Template"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h3>{"Libraries used in this template"}</h3>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <ul>
                        <li><a href="https://yew.rs" target="_blank">{"yew.rs"}</a>{" : rustwasm frontent framwork"}</li>
                        <li><a href="https://github.com/spielrs/yew_styles" target="_blank">
                            {"yew_styles"}</a>{" : styles framework for yew"}</li>
                        <li><a href="https://github.com/spielrs/spielrs-diff" target="_blank">
                            {"spielrs_diff"}</a>{" : diff paths"}</li>
                        <li><a href="https://github.com/seanmonstar/warp" target="_blank">
                            {"warp"}</a>{" : web sever framework"}</li>
                    </ul>
                </Item>
            </Container>
        }
    }
}
