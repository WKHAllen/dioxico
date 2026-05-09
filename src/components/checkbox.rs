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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the checkbox content.
    #[props(default, into)]
    content_class: String,
) -> Element {
    rsx! {
        div {
            class: classes!("dioxico-checkbox-container", class),

            label {
                class: classes!("dioxico-checkbox", disabled.then_some("dioxico-checkbox-disabled")),

                div {
                    class: classes!("dioxico-checkbox-label", content_class),

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
