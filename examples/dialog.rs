use dioxico::{Button, Dialog, DialogActionsLayout, DialogSize, Select};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut dialog_small_state = use_signal(|| false);
    let mut dialog_medium_state = use_signal(|| false);
    let mut dialog_large_state = use_signal(|| false);
    let mut dialog_max_state = use_signal(|| false);
    let mut dialog_auto_state = use_signal(|| false);
    let mut dialog_close_state = use_signal(|| None);
    let dialog_select_state = use_signal(|| 0usize);
    let dialog_select_options = (1..=5)
        .map(|index| format!("Option {}", index))
        .collect::<Vec<_>>();

    rsx! {
        Button {
            text: "Open small dialog",
            on_click: move |_| dialog_small_state.set(true),
        }
        Dialog {
            state: dialog_small_state,
            size: DialogSize::Small,
            title: "Small dialog",
            ok_label: "OK",
            cancel_label: "Cancel",
            on_close_request: move |ok| dialog_close_state.set(Some(ok)),
            actions_layout: DialogActionsLayout::Left,

            p { "A small dialog with left-aligned actions." }
        }
        Button {
            text: "Open medium dialog",
            on_click: move |_| dialog_medium_state.set(true),
        }
        Dialog {
            state: dialog_medium_state,
            size: DialogSize::Medium,
            title: "Medium dialog",
            ok_label: "OK",
            cancel_label: "Cancel",
            on_close_request: move |ok| dialog_close_state.set(Some(ok)),
            actions_layout: DialogActionsLayout::Right,

            p { "A medium dialog with right-aligned actions." }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
            p { "Test" }
            p { "Scrolling" }
            p { "Behavior" }
        }
        Button {
            text: "Open large dialog",
            on_click: move |_| dialog_large_state.set(true),
        }
        Dialog {
            state: dialog_large_state,
            size: DialogSize::Large,
            title: "Large dialog",
            ok_label: "OK",
            cancel_label: "Cancel",
            on_close_request: move |ok| dialog_close_state.set(Some(ok)),
            actions_layout: DialogActionsLayout::Spaced,

            p { "A large dialog with spaced actions." }

            Select {
                state: dialog_select_state,
                options: dialog_select_options,
                label: "Dialog select label",
            }
        }
        Button {
            text: "Open max dialog",
            on_click: move |_| dialog_max_state.set(true),
        }
        Dialog {
            state: dialog_max_state,
            size: DialogSize::Max,
            title: "Max dialog",
            on_close_request: move |ok| dialog_close_state.set(Some(ok)),

            p { "A maximum size dialog with no actions." }
        }
        Button {
            text: "Open auto dialog",
            on_click: move |_| dialog_auto_state.set(true),
        }
        Dialog {
            state: dialog_auto_state,
            size: DialogSize::Auto,
            title: "Auto dialog",
            ok_label: "OK",
            on_close_request: move |ok| dialog_close_state.set(Some(ok)),

            p { "An auto size dialog with only an OK action." }
        }
        span {
            "Close value: {dialog_close_state():?}"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
