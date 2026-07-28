//! Icon components and utilities.

use crate::classes::*;
use crate::css_repr::CssRepr;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// Angle down icon.
#[allow(clippy::volatile_composites)]
pub const ANGLE_DOWN_ICON: Asset = asset!("/src/assets/svg/angle-down-solid.svg");

/// Angle left icon.
#[allow(clippy::volatile_composites)]
pub const ANGLE_LEFT_ICON: Asset = asset!("/src/assets/svg/angle-left-solid.svg");

/// Angle right icon.
#[allow(clippy::volatile_composites)]
pub const ANGLE_RIGHT_ICON: Asset = asset!("/src/assets/svg/angle-right-solid.svg");

/// Calendar icon.
#[allow(clippy::volatile_composites)]
pub const CALENDAR_ICON: Asset = asset!("/src/assets/svg/calendar-days-solid.svg");

/// Checkmark icon.
#[allow(clippy::volatile_composites)]
pub const CHECK_ICON: Asset = asset!("/src/assets/svg/check-solid.svg");

/// X mark icon.
#[allow(clippy::volatile_composites)]
pub const XMARK_ICON: Asset = asset!("/src/assets/svg/xmark-solid.svg");

/// The size of an icon.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum IconSize {
    /// A small icon.
    Small,
    /// A medium icon.
    #[default]
    Medium,
    /// A large icon.
    Large,
}

/// Icon component.
#[component]
pub fn Icon(
    /// Icon path.
    icon: Asset,
    /// Icon size.
    #[props(default)]
    size: IconSize,
    /// Is this icon disabled?
    #[props(default)]
    disabled: bool,
    /// CSS classes to apply to the icon.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
      img {
        class: classes!(
            "dioxico-icon", format!("dioxico-icon-{}", size.css_repr()), disabled
            .then_some("dioxico-icon-disabled"), class
        ),
        src: icon,
      }
    }
}
