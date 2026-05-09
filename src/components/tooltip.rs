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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the tooltip content container.
    #[props(default, into)]
    content_class: String,
    /// CSS classes to apply to the tooltip hover text.
    #[props(default, into)]
    text_class: String,
) -> Element {
    let mut hovering_state = use_signal(|| false);

    rsx! {
        div {
            class: classes!("dioxico-tooltip", hovering_state().then_some("dioxico-tooltip-open"), disabled.then_some("dioxico-tooltip-disabled"), class),

            div {
                class: classes!("dioxico-tooltip-content", content_class),
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
                            class: classes!("dioxico-tooltip-text", text_class),

                            {text}
                        }
                    }
                }
            }
        }
    }
}
