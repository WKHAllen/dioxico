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
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Smaller => "smaller",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Larger => "larger",
        }
    }
}

/// Error properties.
#[derive(Props)]
pub struct ErrorProps<'a> {
    /// The error message.
    #[props(!optional, default)]
    message: Option<&'a str>,
    /// The size of the error message.
    #[props(default)]
    size: ErrorSize,
}

/// An error element.
pub fn Error<'a>(cx: Scope<'a, ErrorProps<'a>>) -> Element {
    let message = cx.props.message.unwrap_or_default();
    let class = classes!(
        "dioxico-error",
        format!("dioxico-text-{}", cx.props.size.size_name())
    );

    cx.render(rsx! {
        span {
            class: "{class}",

            "{message}"
        }
    })
}
