//! Badge components and utilities.

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
    pub const fn as_str(&self) -> &'static str {
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the value text.
    #[props(default, into)]
    value_class: String,
) -> Element
where
    N: Number + 'static,
{
    rsx! {
        div {
            class: classes!("dioxico-badge-container", class),

            div {
                class: classes!("dioxico-badge", format!("dioxico-badge-{}", style.as_str())),

                div {
                    class: classes!("dioxico-badge-text", value_class),

                    {value.to_string()}
                }
            }
        }
    }
}
