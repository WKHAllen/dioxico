use crate::classes::*;
use dioxus::prelude::*;

/// The size of an error message.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
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

impl ErrorSize {
    /// Gets the name of the error message size.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Smaller => "smaller",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Larger => "larger",
        }
    }
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
) -> Element {
    rsx! {
        span {
            class: classes!("dioxico-error", format!("dioxico-text-{}", size.as_str())),

            "{message}"
        }
    }
}
