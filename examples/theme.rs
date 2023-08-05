use dioxico::use_theme;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Demo(cx: Scope) -> Element {
    let _theme = use_theme(cx);

    cx.render(rsx! {
        div {
            "Theme customization placeholder"
        }
    })
}

#[allow(dead_code)]
fn main() {
    dioxus_web::launch(Demo)
}
