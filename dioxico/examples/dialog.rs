use dioxico::{Button, Dialog, DialogActionsLayout, DialogSize, Input, Select, sleep};
use dioxus::prelude::*;
use std::rc::Rc;
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut dialog_small_state = use_signal(|| false);
    let mut dialog_medium_state = use_signal(|| false);
    let mut dialog_large_state = use_signal(|| false);
    let mut dialog_max_state = use_signal(|| false);
    let mut dialog_auto_state = use_signal(|| false);
    let mut dialog_close_state = use_signal(|| None);
    let dialog_input_state = use_signal(String::new);
    let dialog_select_state = use_signal(|| 0usize);
    let dialog_select_options = (1..=5)
        .map(|index| format!("Option {}", index))
        .collect::<Vec<_>>();
    let mut dialog_input_data = use_signal(|| None::<Rc<MountedData>>);

    rsx! {
      Button {
        on_click: move |_| async move {
            dialog_small_state.set(true);
            if let Some(input_data) = &*dialog_input_data.read() {
                sleep(Duration::from_secs_f64(0.1)).await;
                let _ = input_data.set_focus(true).await;
            }
        },
        "Open small dialog"
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
        Input {
          state: dialog_input_state,
          placeholder: "This auto-focuses when the dialog opens",
          on_mounted: move |event: Event<MountedData>| dialog_input_data.set(Some(event.data())),
        }
      }
      Button { on_click: move |_| dialog_medium_state.set(true), "Open medium dialog" }
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
      Button { on_click: move |_| dialog_large_state.set(true), "Open large dialog" }
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
      Button { on_click: move |_| dialog_max_state.set(true), "Open max dialog" }
      Dialog {
        state: dialog_max_state,
        size: DialogSize::Max,
        title: "Max dialog",
        on_close_request: move |ok| dialog_close_state.set(Some(ok)),

        p { "A maximum size dialog with no actions." }
      }
      Button { on_click: move |_| dialog_auto_state.set(true), "Open auto dialog" }
      Dialog {
        state: dialog_auto_state,
        size: DialogSize::Auto,
        title: "Auto dialog",
        ok_label: "OK",
        on_close_request: move |ok| dialog_close_state.set(Some(ok)),

        p { "An auto size dialog with only an OK action." }
      }
      span { "Close value: {dialog_close_state():?}" }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
