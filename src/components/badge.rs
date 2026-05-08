use crate::classes::*;
use crate::util::*;
use dioxus::prelude::*;

/// Badge style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BadgeStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Danger style.
    Danger,
}

impl BadgeStyle {
    /// Gets the name of the badge style.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Danger => "danger",
        }
    }
}

/// Badge component.
#[component]
pub fn Badge<N>(
    /// Badge value.
    value: N,
    /// Badge style.
    #[props(default)]
    style: BadgeStyle,
) -> Element
where
    N: Number + 'static,
{
    rsx! {
        div {
            class: "dioxico-badge-container",

            div {
                class: classes!("dioxico-badge", format!("dioxico-badge-{}", style.as_str())),

                div {
                    class: "dioxico-badge-text",

                    {value.to_string()}
                }
            }
        }
    }
}
