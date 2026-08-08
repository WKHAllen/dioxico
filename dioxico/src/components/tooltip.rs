//! Tooltip popup components and utilities.

use crate::classes;
use crate::element::ElementLike;
use dioxus::prelude::*;

/// Tooltip popup component.
#[component]
pub fn Tooltip(
    /// Tooltip popup content.
    #[props(into)]
    content: ElementLike,
    /// Is this tooltip disabled?
    #[props(default)]
    disabled: bool,
    /// Elements within the tooltip hover area.
    children: Element,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the tooltip inner container.
    #[props(default, into)]
    inner_class: String,
    /// CSS classes to apply to the tooltip hover text.
    #[props(default, into)]
    content_class: String,
) -> Element {
    let mut hovering_state = use_signal(|| false);

    rsx! {
      div {
        class: classes!(
            "dioxico-tooltip", hovering_state().then_some("dioxico-tooltip-open"), disabled
            .then_some("dioxico-tooltip-disabled"), class
        ),

        div {
          class: classes!("dioxico-tooltip-inner", inner_class),
          onmouseenter: move |_| hovering_state.set(true),
          onmouseleave: move |_| hovering_state.set(false),

          {children}
        }

        div { class: "dioxico-tooltip-container",
          div { class: "dioxico-tooltip-popup-container",
            div { class: "dioxico-tooltip-popup",
              // node ref?
              span { class: classes!("dioxico-tooltip-content", content_class),
                {content}
              }
            }
          }
        }
      }
    }
}
