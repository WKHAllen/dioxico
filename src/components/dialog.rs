use super::{Button, ButtonStyle, IconButton, IconButtonSize, XMARK_ICON};
use crate::classes::*;
use crate::state::State;
use dioxus::prelude::*;

/// Dialog size.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogSize {
    /// Small dialog.
    Small,
    /// Medium size dialog.
    #[default]
    Medium,
    /// Large dialog.
    Large,
    /// Maximum size dialog.
    Max,
    /// Dialog sized to fit the inner content.
    Auto,
}

impl DialogSize {
    /// Gets the name of the dialog size.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
            Self::Auto => "auto",
        }
    }
}

/// Dialog action buttons layout.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogActionsLayout {
    /// Left-aligned actions.
    Left,
    /// Right-aligned actions.
    #[default]
    Right,
    /// Actions spaced across the line.
    Spaced,
}

impl DialogActionsLayout {
    /// Gets the name of the actions layout.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Spaced => "spaced",
        }
    }
}

/// Dialog popup component.
#[component]
pub fn Dialog(
    /// Dialog open state.
    #[props(into)]
    state: State<bool>,
    /// Dialog size.
    #[props(default)]
    size: DialogSize,
    /// Dialog title text.
    #[props(default)]
    title: String,
    /// Ok button label. Will not be created if empty.
    #[props(default)]
    ok_label: String,
    /// Cancel button label. Will not be created if empty.
    #[props(default)]
    cancel_label: String,
    /// Callback called with the dialog closing state. Receives `true` if the ok
    /// button was clicked and `false` otherwise.
    #[props(default)]
    on_close_request: EventHandler<bool>,
    /// Should this dialog be closed when the ok button is clicked? Defaults to
    /// `true`.
    #[props(default = true)]
    close_on_ok: bool,
    /// Should this dialog be closed when the cancel button is clicked? Defaults
    /// to `true`.
    #[props(default = true)]
    close_on_cancel: bool,
    /// The layout of the action buttons.
    #[props(default)]
    actions_layout: DialogActionsLayout,
    /// Elements within the dialog.
    children: Element,
) -> Element {
    let mut mouse_in_state = use_signal(|| false);

    rsx! {
        div {
            class: classes!("dioxico-dialog-container", state.get().then_some("dioxico-dialog-container-open")),
            onclick: move |_| {
                if !mouse_in_state() {
                    on_close_request.call(false);
                    state.set(false);
                }
            },

            div {
                class: classes!("dioxico-dialog", format!("dioxico-dialog-{}", size.as_str())),
                onmouseenter: move |_| mouse_in_state.set(true),
                onmouseleave: move |_| mouse_in_state.set(false),

                div {
                    class: "dioxico-dialog-inner",

                    div {
                        class: "dioxico-dialog-header",

                        div {
                            class: "dioxico-dialog-header-space",
                        }

                        h3 {
                            class: "dioxico-dialog-title",

                            {title}
                        }

                        IconButton {
                            icon: XMARK_ICON,
                            size: IconButtonSize::Medium,
                            on_click: move |_| {
                                on_close_request.call(false);
                                state.set(false);
                            },
                        }
                    }

                    div {
                        class: "dioxico-dialog-body",

                        {children}
                    }

                    div {
                        class: classes!("dioxico-dialog-actions", format!("dioxico-dialog-actions-{}", actions_layout.as_str())),

                        if !cancel_label.is_empty() {
                            Button {
                                text: cancel_label,
                                style: ButtonStyle::Transparent,
                                on_click: move |_| {
                                    on_close_request.call(false);

                                    if close_on_cancel {
                                        state.set(false);
                                    }
                                },
                            }
                        }

                        if !ok_label.is_empty() {
                            Button {
                                text: ok_label,
                                style: ButtonStyle::Primary,
                                on_click: move |_| {
                                    on_close_request.call(true);

                                    if close_on_ok {
                                        state.set(false);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
