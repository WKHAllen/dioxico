//! Dialog components and utilities.

use super::{Button, ButtonStyle, IconButton, IconButtonSize, XMARK_ICON};
use crate::classes;
use crate::css_repr::CssRepr;
use crate::element::ElementLike;
use crate::state::State;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// Dialog size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
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

/// Dialog action buttons layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum DialogActionsLayout {
    /// Left-aligned actions.
    Left,
    /// Right-aligned actions.
    #[default]
    Right,
    /// Actions spaced across the line.
    Spaced,
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
    /// Dialog title element.
    #[props(default, into)]
    title: ElementLike,
    /// Ok button label. Will not be created if empty.
    #[props(default, into)]
    ok_label: ElementLike,
    /// Cancel button label. Will not be created if empty.
    #[props(default, into)]
    cancel_label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
    /// CSS classes to apply to the dialog content.
    #[props(default, into)]
    content_class: String,
    /// Elements within the dialog.
    children: Element,
) -> Element {
    let mut mouse_in_state = use_signal(|| false);

    rsx! {
        div {
            class: classes!(
                "dioxico-dialog-container", state.get()
                .then_some("dioxico-dialog-container-open"), class
            ),
            onclick: move |_| {
                if !mouse_in_state() {
                    on_close_request.call(false);
                    state.set(false);
                }
            },

            div {
                class: classes!("dioxico-dialog", format!("dioxico-dialog-{}", size.css_repr())),
                onmouseenter: move |_| mouse_in_state.set(true),
                onmouseleave: move |_| mouse_in_state.set(false),

                div { class: "dioxico-dialog-inner",

                    div { class: "dioxico-dialog-header",

                        div { class: "dioxico-dialog-header-space" }

                        h3 { class: "dioxico-dialog-title", {title} }

                        IconButton {
                            icon: XMARK_ICON,
                            size: IconButtonSize::Medium,
                            on_click: move |_| {
                                on_close_request.call(false);
                                state.set(false);
                            },
                        }
                    }

                    div { class: classes!("dioxico-dialog-body", content_class), {children} }

                    div {
                        class: classes!(
                            "dioxico-dialog-actions", format!("dioxico-dialog-actions-{}", actions_layout
                            .css_repr())
                        ),

                        if !cancel_label.is_empty() {
                            Button {
                                style: ButtonStyle::Transparent,
                                on_click: move |_| {
                                    on_close_request.call(false);

                                    if close_on_cancel {
                                        state.set(false);
                                    }
                                },

                                {cancel_label}
                            }
                        }

                        if !ok_label.is_empty() {
                            Button {
                                style: ButtonStyle::Primary,
                                on_click: move |_| {
                                    on_close_request.call(true);

                                    if close_on_ok {
                                        state.set(false);
                                    }
                                },

                                {ok_label}
                            }
                        }
                    }
                }
            }
        }
    }
}
