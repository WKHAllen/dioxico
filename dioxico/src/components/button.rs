//! Button components and utilities.

use crate::classes;
use crate::css_repr::CssRepr;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// The style of a button.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum ButtonStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Transparent style.
    Transparent,
    /// Danger style.
    Danger,
}

/// Button component.
#[component]
pub fn Button(
    /// Button style.
    #[props(default)]
    style: ButtonStyle,
    /// Is this button disabled?
    #[props(default)]
    disabled: bool,
    /// Callback called when the button is clicked.
    #[props(default)]
    on_click: EventHandler<()>,
    /// Callback called when the button element is mounted.
    #[props(default)]
    on_mounted: EventHandler<Event<MountedData>>,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// Button inner elements.
    children: Element,
) -> Element {
    rsx! {
      button {
        r#type: "button",
        class: classes!("dioxico-button", format!("dioxico-button-{}", style.css_repr()), class),
        onclick: move |event| {
            event.prevent_default();

            if !disabled {
                on_click.call(());
            }
        },
        onmounted: on_mounted,
        disabled,

        {children}
      }
    }
}
