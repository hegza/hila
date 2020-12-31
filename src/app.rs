use super::{Activated, Hovered, WeakComponentLink};
use crate::widget::attention_circle::AttentionCircle;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    Activate(Activated),
    Hover(Hovered),
}

pub struct App {
    link: ComponentLink<Self>,
    circle_link: WeakComponentLink<AttentionCircle>,
    hovered: Hovered,
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            hovered: Hovered::None,
            circle_link: WeakComponentLink::default(),
            value: 0,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::Activate(Activated) => {
                println!("big circle activated at app.rs!")
            }
            Msg::Hover(hovered) => {
                self.hovered = hovered;
            }
        };
        true
    }

    fn view(&self) -> Html {
        let on_click = &self.link.callback(Msg::Activate);
        let on_hover = &self.link.callback(Msg::Hover);
        let circle_link = &self.circle_link;

        html! {
            <div class="main">
                <AttentionCircle on_click=on_click on_hover=on_hover weak_link=circle_link>
                </AttentionCircle>
            </div>
        }
    }
}
