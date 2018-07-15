#[macro_use]
extern crate stdweb;
extern crate yew;
extern crate my_proj;

use yew::prelude::*;
use stdweb::web::{IElement, INode, IParentNode, document};
use my_proj::Model;

fn main() {
    yew::initialize();
    let body = document().query_selector("body").unwrap().unwrap();

    let mount_class = "mount-point";
    let mount_point = document().create_element("div").unwrap();
    mount_point.class_list().add(mount_class).unwrap();
    body.append_child(&mount_point);

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}
