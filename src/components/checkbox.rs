use super::CHECK_ICON;
use crate::classes::*;
use crate::state::State;
use dioxus::prelude::*;

/// Checkbox component.
#[component]
pub fn Checkbox(
    /// Checkbox state.
    #[props(into)]
    state: State<bool>,
    /// Checkbox label.
    #[props(default)]
    label: String,
    /// Is this checkbox disabled?
    #[props(default)]
    disabled: bool,
) -> Element {
    rsx! {
        div {
            class: "dioxico-checkbox-container",

            label {
                class: classes!("dioxico-checkbox", disabled.then_some("dioxico-checkbox-disabled")),

                span {
                    class: "dioxico-checkbox-label",

                    {label}
                }

                input {
                    r#type: "checkbox",
                    class: "dioxico-checkbox-input",
                    checked: state.get(),
                    oninput: move |event| state.set(event.checked()),
                    disabled,
                }

                span {
                    class: "dioxico-checkmark",

                    img {
                        class: "dioxico-checkmark-icon",
                        src: CHECK_ICON,
                    }
                }
            }
        }
    }
}
