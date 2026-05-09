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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the switch content.
    #[props(default, into)]
    content_class: String,
) -> Element {
    rsx! {
        div {
            class: classes!("dioxico-switch-container", class),

            label {
                class: classes!("dioxico-switch", disabled.then_some("dioxico-switch-disabled")),

                div {
                    class: classes!("dioxico-switch-label", content_class),

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
