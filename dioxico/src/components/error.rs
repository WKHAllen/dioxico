//! Error components and utilities.

use crate::classes;
use crate::css_repr::CssRepr;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// The size of an error message.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum ErrorSize {
    /// A very small message.
    Smaller,
    /// A small message.
    Small,
    /// A medium sized message.
    #[default]
    Medium,
    /// A large message.
    Large,
    /// A very large message.
    Larger,
}

/// An error element.
#[component]
pub fn Error(
    /// Error message. If missing or empty, no error will be shown.
    #[props(into)]
    message: String,
    /// Error message size.
    #[props(default)]
    size: ErrorSize,
    /// CSS classes to apply to the error.
    #[props(default, into)]
    class: String,
) -> Element {
    let empty = message.is_empty();

    rsx! {
      span {
        class: classes!(
            "dioxico-error", format!("dioxico-text-{}", size.css_repr()), empty
            .then_some("dioxico-error-empty"), class
        ),
        {message}
      }
    }
}
