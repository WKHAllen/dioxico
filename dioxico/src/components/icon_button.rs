//! Icon button components and utilities.

use super::IconSize;
use crate::classes::*;
use crate::css_repr::CssRepr;
use dioxus::prelude::*;

/// The size of an icon button.
pub type IconButtonSize = IconSize;

/// Icon button component.
#[component]
pub fn IconButton(
    /// Icon asset.
    icon: Asset,
    /// Icon button size.
    #[props(default)]
    size: IconButtonSize,
    /// Is this icon button disabled?
    #[props(default)]
    disabled: bool,
    /// Callback called when the icon button is clicked.
    #[props(default)]
    on_click: EventHandler<()>,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the icon.
    #[props(default, into)]
    icon_class: String,
) -> Element {
    rsx! {
      button {
        r#type: "button",
        class: classes!(
            "dioxico-icon-button", format!("dioxico-icon-button-{}", size.css_repr()), class
        ),
        onclick: move |_| {
            if !disabled {
                on_click.call(());
            }
        },
        disabled,

        img {
          class: classes!("dioxico-icon-button-icon", icon_class),
          src: icon,
        }
      }
    }
}
