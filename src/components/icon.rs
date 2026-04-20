use crate::classes::*;
use dioxus::prelude::*;

/// Angle down icon.
pub const ANGLE_DOWN_ICON: Asset = asset!("/src/assets/svg/angle-down-solid.svg");

/// Angle left icon.
pub const ANGLE_LEFT_ICON: Asset = asset!("/src/assets/svg/angle-left-solid.svg");

/// Angle right icon.
pub const ANGLE_RIGHT_ICON: Asset = asset!("/src/assets/svg/angle-right-solid.svg");

/// Calendar icon.
pub const CALENDAR_ICON: Asset = asset!("/src/assets/svg/calendar-days-solid.svg");

/// Checkmark icon.
pub const CHECK_ICON: Asset = asset!("/src/assets/svg/check-solid.svg");

/// X mark icon.
pub const XMARK_ICON: Asset = asset!("/src/assets/svg/xmark-solid.svg");

/// The size of an icon.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconSize {
    /// A small icon.
    Small,
    /// A medium icon.
    #[default]
    Medium,
    /// A large icon.
    Large,
}

impl IconSize {
    /// Gets the name of the icon size.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
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
            class: classes!("dioxico-icon", format!("dioxico-icon-{}", size.as_str()), disabled.then_some("dioxico-icon-disabled"), class),
            src: icon,
        }
    }
}
