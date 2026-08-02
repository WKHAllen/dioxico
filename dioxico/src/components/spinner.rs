//! Loading spinner components and utilities.

use crate::classes;
use crate::css_repr::CssRepr;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// The size of a spinner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum SpinnerSize {
    /// A small spinner.
    Small,
    /// A medium size spinner.
    #[default]
    Medium,
    /// A large spinner.
    Large,
    /// A spinner that grows to the size of the container.
    Max,
}

/// Loading spinner component.
#[component]
pub fn Spinner(
    /// Spinner size.
    #[props(default)]
    size: SpinnerSize,
    /// Should the spinner be centered?
    #[props(default = true)]
    center: bool,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
        div {
            class: classes!(
                "dioxico-spinner-container", center.then_some("dioxico-spinner-center"), class
            ),

            svg {
                class: classes!("dioxico-spinner", format!("dioxico-spinner-{}", size.css_repr())),
                view_box: "0 0 50 50",

                circle {
                    class: "path",
                    cx: 25,
                    cy: 25,
                    r: 20,
                    fill: "none",
                    stroke_width: 5,
                }
            }
        }
    }
}
