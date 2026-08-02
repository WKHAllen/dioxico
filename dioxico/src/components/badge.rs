//! Badge components and utilities.

use crate::classes;
use crate::css_repr::CssRepr;
use crate::util::*;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// Badge style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum BadgeStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Danger style.
    Danger,
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
        div { class: classes!("dioxico-badge-container", class),

            div { class: classes!("dioxico-badge", format!("dioxico-badge-{}", style.css_repr())),

                div { class: classes!("dioxico-badge-text", value_class), {value.to_string()} }
            }
        }
    }
}
