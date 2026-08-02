//! Accordion components and utilities.

use super::{Icon, IconSize, ANGLE_RIGHT_ICON};
use crate::classes;
use crate::element::ElementLike;
use crate::state::State;
use dioxus::prelude::*;

/// An expandable/collapsible accordion component.
#[component]
pub fn Accordion(
    /// The open/closed state of the accordion.
    #[props(default, into)]
    state: State<bool>,
    /// Accordion title element.
    #[props(into)]
    title: ElementLike,
    /// Is this accordion disabled?
    #[props(default)]
    disabled: bool,
    /// Elements within the expandable section.
    children: Element,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the accordion content.
    #[props(default, into)]
    content_class: String,
) -> Element {
    rsx! {
      div {
        class: classes!(
            "dioxico-accordion", state.get().then_some("dioxico-accordion-open"), disabled
            .then_some("dioxico-accordion-disabled"), class
        ),

        div {
          class: "dioxico-accordion-header",
          onclick: move |_| {
              if !disabled {
                  state.set(!state.get());
              }
          },

          Icon {
            icon: ANGLE_RIGHT_ICON,
            size: IconSize::Medium,
            disabled,
            class: "dioxico-accordion-header-icon",
          }

          div { class: "dioxico-accordion-header-title", {title} }
        }

        div { class: classes!("dioxico-accordion-content", content_class), {children} }
      }
    }
}
