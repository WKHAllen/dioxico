use crate::classes::*;
use dioxus::prelude::*;

/// Tooltip popup component.
#[component]
pub fn Tooltip(
    /// Tooltip popup text.
    #[props(into)]
    text: String,
    /// Is this tooltip disabled?
    #[props(default)]
    disabled: bool,
    /// Elements within the tooltip hover area.
    children: Element,
    /// CSS classes to apply to the tooltip container.
    #[props(default, into)]
    class: String,
) -> Element {
    let mut hovering_state = use_signal(|| false);

    rsx! {
        div {
            class: classes!("dioxico-tooltip", hovering_state().then_some("dioxico-tooltip-open"), disabled.then_some("dioxico-tooltip-disabled")),

            div {
                class: classes!("dioxico-tooltip-content", class),
                onmouseenter: move |_| hovering_state.set(true),
                onmouseleave: move |_| hovering_state.set(false),

                {children}
            }

            div {
                class: "dioxico-tooltip-container",

                div {
                    class: "dioxico-tooltip-popup-container",

                    div {
                        class: "dioxico-tooltip-popup",
                        // node ref?

                        span {
                            class: "dioxico-tooltip-text",

                            {text}
                        }
                    }
                }
            }
        }
    }
}
