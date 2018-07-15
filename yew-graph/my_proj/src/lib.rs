#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;

use yew::prelude::*;
use stdweb::web::{IElement, INode, IParentNode, document};

pub struct Model {
    name: String,
}

pub enum Msg {
    UpdateName(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let body = document().query_selector("body").unwrap().unwrap();

        // This canvas won't be overwritten by yew!
        let canvas = document().create_element("canvas").unwrap();
        body.append_child(&canvas);

        js! {
        const canvas = document.querySelector("canvas");
        canvas.width = 100;
        canvas.height = 100;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 50, 50);
        };

        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input value=&self.name, oninput=|e| Msg::UpdateName(e.value), />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}
