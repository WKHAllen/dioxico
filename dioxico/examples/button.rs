use dioxico::{Button, ButtonStyle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut last_button_clicked_state = use_signal(|| None);

    rsx! {
        Button {
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Primary)),

            "Primary"
        }
        Button {
            style: ButtonStyle::Secondary,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Secondary)),

            "Secondary"
        }
        Button {
            style: ButtonStyle::Transparent,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Transparent)),

            "Transparent"
        }
        Button {
            style: ButtonStyle::Danger,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Danger)),

            "Danger"
        }
        Button {
            style: last_button_clicked_state().unwrap_or_default(),
            disabled: true,

            "Disabled"
        }
        span {
            "Last clicked: {last_button_clicked_state():?}"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
