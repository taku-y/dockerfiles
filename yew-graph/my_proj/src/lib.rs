#![recursion_limit="256"]

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
        let xs = vec![5.0, 2.0, 10.0, 3.0, 5.0, 4.0, 5.0, 5.0, 2.5, 6.0];
        let ys = vec![3.0, 1.0, 7.0, 5.0, 10.0, 6.0, 4.0, 4.0, 2.0, 1.0];

        js! {
            console.log("start");
            Plotly.plot(
                document.getElementById("plot_area"),
                [{x: @{xs}, y: @{ys}}],
                {margin: {t: 0}}
            );
            console.log("end");
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
