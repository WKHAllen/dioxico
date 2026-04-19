use dioxico::{Button, ButtonStyle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut last_button_clicked_state = use_signal(|| None);

    rsx! {
        Button {
            text: "Primary",
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Primary)),
        }
        Button {
            text: "Secondary",
            style: ButtonStyle::Secondary,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Secondary)),
        }
        Button {
            text: "Transparent",
            style: ButtonStyle::Transparent,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Transparent)),
        }
        Button {
            text: "Danger",
            style: ButtonStyle::Danger,
            on_click: move |_| last_button_clicked_state.set(Some(ButtonStyle::Danger)),
        }
        Button {
            text: "Disabled",
            style: last_button_clicked_state().unwrap_or_default(),
            disabled: true,
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
