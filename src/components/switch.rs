use crate::classes::*;
use crate::state::State;
use dioxus::prelude::*;

/// Switch component.
#[component]
pub fn Switch(
    /// Switch state.
    #[props(into)]
    state: State<bool>,
    /// Switch label.
    #[props(default)]
    label: String,
    /// Is this switch disabled?
    #[props(default)]
    disabled: bool,
) -> Element {
    rsx! {
        div {
            class: "dioxico-switch-container",

            label {
                class: classes!("dioxico-switch", disabled.then_some("dioxico-switch-disabled")),

                span {
                    class: "dioxico-switch-label",

                    {label}
                }

                input {
                    r#type: "checkbox",
                    class: "dioxico-switch-input",
                    checked: state.get(),
                    oninput: move |event| state.set(event.checked()),
                    disabled,
                }

                span {
                    class: "dioxico-switch-toggle",
                }
            }
        }
    }
}
