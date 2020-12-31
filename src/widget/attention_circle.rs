use crate::{Activated, Hovered, WeakComponentLink};
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

pub enum Msg {
    // Click or key-press on the big circle
    Activate,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_click: Callback<Activated>,
    pub on_hover: Callback<Hovered>,
    pub weak_link: WeakComponentLink<AttentionCircle>,
}

pub struct AttentionCircle {
    props: Props,
    inactive: bool,
}

impl Component for AttentionCircle {
    type Message = Msg;
    type Properties = Props;

    fn create(props: <Self as yew::Component>::Properties, link: yew::html::Scope<Self>) -> Self {
        props.weak_link.borrow_mut().replace(link);

        Self {
            props,
            inactive: false,
        }
    }

    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            Msg::Activate => {
                println!("big circle activated at attention_circle.rs!");
                true
            }
        }
    }

    fn change(&mut self, props: <Self as yew::Component>::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        let inactive = if self.inactive { "inactive" } else { "" };
        let onmouseover = self.props.on_hover.reform(|_| Hovered::AttentionCircle);
        let onmouseout = self.props.on_hover.reform(|_| Hovered::None);
        let onclick = self.props.on_click.reform(|_| Activated);

        html! {
            <div class="circle-widget" onmouseout=onmouseout onmouseover=onmouseover onclick=onclick>
                // Outer-circle, background
                <div class="circle" style="background:var(--a2)">
                    // Inner circle, "button"
                    <div class="circle" style="width:90%;height:90%;background:var(--m1);">{
                        html!{<div>
                            <h1>{"Example title text"}</h1>
                            <h2>{"Example key text"}</h2>
                        </div>}
                    }</div>
                </div>
            </div>
        }

        /*html!{
            <h1>{Example title text}</h1>
        }
        html!{
            <h2>{Example key text}</h2>
        }*/
    }
}
