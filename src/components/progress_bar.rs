use crate::classes::*;
use dioxus::prelude::*;

/// Progress bar component.
#[component]
pub fn ProgressBar(
    /// Progress as a value between 0 and 1.
    #[props(into)]
    progress: f64,
    /// Is this progress bar disabled?
    #[props(default)]
    disabled: bool,
) -> Element {
    let width_style = format!("width: {}%;", progress * 100.0);

    rsx! {
        div {
            class: classes!("dioxico-progress", disabled.then_some("dioxico-progress-disabled")),

            div {
                class: "dioxico-progress-empty",
            }

            div {
                class: "dioxico-progress-filled",
                style: width_style,
            }
        }
    }
}
