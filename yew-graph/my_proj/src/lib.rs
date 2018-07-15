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
        let body = document().query_selector("body").unwrap().unwrap();
        let labels = vec!["2008","2009","2010","2011","2012","2013","2014","2015","2016","2017"];
        let xs = vec![5.0, 2.0, 10.0, 3.0, 5.0, 4.0, 5.0, 5.0, 2.5, 6.0];
        let ys = vec![3.0, 1.0, 7.0, 5.0, 10.0, 6.0, 4.0, 4.0, 2.0, 1.0];
        let data = vec![xs, ys];

        // This canvas won't be overwritten by yew!
        let canvas = document().create_element("canvas").unwrap();
        body.append_child(&canvas);

        js! {
            line = new RGraph.Line({
                id:"cvs",
                data: @{data},
                options: {
                    colors: ["red", "white"],
                    backgroundGrid: false,
                    axisColor: "#ccc",
                    textColor: "#ccc",
                    tickmarks: "filledcircle",
                    tickmarksSize: 50,
                    linewidth:3 ,
                    labels: @{labels},
                    gutterLeft: 50,
                    gutterRight: 50,
                    gutterTop: 50,
                    gutterBottom: 50,
                },
            }).trace();
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
