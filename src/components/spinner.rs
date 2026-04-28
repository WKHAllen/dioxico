use crate::classes::*;
use dioxus::prelude::*;

/// The size of a spinner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl SpinnerSize {
    /// Gets the name of the spinner size.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
        }
    }
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
) -> Element {
    rsx! {
        div {
            class: classes!("dioxico-spinner-container", center.then_some("dioxico-spinner-center")),

            svg {
                class: classes!("dioxico-spinner", format!("dioxico-spinner-{}", size.as_str())),
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
