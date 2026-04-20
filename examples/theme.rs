use dioxico::{use_theme, ColorMode, Switch};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut theme = use_theme();

    rsx! {
        Switch {
            state: (theme.read().color_mode.is_dark(), move |is_dark| {
                theme.write().color_mode = if is_dark { ColorMode::Dark } else { ColorMode::Light };
            }),
            label: "Color mode",
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
