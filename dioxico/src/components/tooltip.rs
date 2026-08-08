//! Tooltip popup components and utilities.

use super::{Popover, PopoverPositionY, PopoverType};
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
    /// CSS classes to apply to the content element.
    #[props(default, into)]
    content_class: String,
) -> Element {
    let mut state = use_signal(|| false);

    rsx! {
      div {
        class: "dioxico-tooltip",
        onmouseenter: move |_| {
            if !disabled {
                state.set(true);
            }
        },
        onmouseleave: move |_| state.set(false),
        Popover {
          state,
          popover_type: PopoverType::Manual,
          anchored: true,
          anchor_class: classes!("dioxico-tooltip-anchor", class),
          anchor_content: children,
          position_y: PopoverPositionY::Bottom,
          class: classes!("dioxico-tooltip-popover", content_class),
          {content}
        }
      }
    }
}
