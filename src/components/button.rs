use crate::classes::*;
use dioxus::prelude::*;

/// The style of a button.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl ButtonStyle {
    /// Gets the name of the button style.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Transparent => "transparent",
            Self::Danger => "danger",
        }
    }
}

/// Button component.
#[component]
pub fn Button(
    /// Button text.
    text: String,
    /// Button style.
    #[props(default)]
    style: ButtonStyle,
    /// Is this button disabled?
    #[props(default)]
    disabled: bool,
    /// Callback called when the button is clicked.
    #[props(default)]
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: classes!("dioxico-button", format!("dioxico-button-{}", style.as_str())),
            onclick: move |_| {
                if !disabled {
                    on_click.call(());
                }
            },
            disabled,

            {text}
        }
    }
}
