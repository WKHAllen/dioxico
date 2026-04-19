use dioxico::use_theme;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let _theme = use_theme();

    // TODO: add switch for light/dark mode

    rsx! {
        div {
            "Theme customization placeholder"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
