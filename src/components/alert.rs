use super::{IconButton, IconButtonSize, XMARK_ICON};
use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;
use std::time::Duration;

/// Alert popup duration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlertDuration {
    /// A finite duration in milliseconds.
    Finite(Duration),
    /// An infinite duration.
    #[default]
    Infinite,
}

impl AlertDuration {
    /// Is the duration infinite?
    pub fn is_infinite(&self) -> bool {
        match *self {
            Self::Finite(_) => false,
            Self::Infinite => true,
        }
    }
}

impl From<Duration> for AlertDuration {
    fn from(value: Duration) -> Self {
        Self::Finite(value)
    }
}

macro_rules! impl_from_secs {
    ( $( $ty:ty )* ) => {
        $(
            impl From<$ty> for AlertDuration {
                fn from(value: $ty) -> Self {
                    Self::Finite(Duration::from_secs(value.into()))
                }
            }
        )*
    };
}

impl_from_secs!(u8 u16 u32 u64);

impl From<f32> for AlertDuration {
    fn from(value: f32) -> Self {
        Self::Finite(Duration::from_secs_f32(value))
    }
}

impl From<f64> for AlertDuration {
    fn from(value: f64) -> Self {
        Self::Finite(Duration::from_secs_f64(value))
    }
}

/// Alert popup component.
#[component]
pub fn Alert(
    /// Alert open state.
    #[props(into)]
    state: State<bool>,
    /// Alert title text.
    #[props(default)]
    title: String,
    /// The duration of time for which the alert should exist. This can be an
    /// [`AlertDuration`], a [`std::time::Duration`], or an int/float
    /// representing the alert duration in seconds. If left empty, it will
    /// default to an infinite alert.
    #[props(default, into)]
    duration: AlertDuration,
    /// Callback called with the alert closing state. Receives `true` if the
    /// alert was closed manually and `false` otherwise.
    #[props(default)]
    on_close: EventHandler<bool>,
    /// Elements within the alert.
    children: Element,
) -> Element {
    let mut timeout_state = use_signal(|| None);

    if state.get() && !duration.is_infinite() && timeout_state.read().is_none() {
        timeout_state.set(match duration {
            AlertDuration::Finite(duration_inner) => {
                if state.get() {
                    Some(spawn(async move {
                        sleep(duration_inner).await;
                        on_close.call(false);
                        state.set(false);
                        timeout_state.set(None);
                    }))
                } else {
                    None
                }
            }
            AlertDuration::Infinite => None,
        })
    }

    rsx! {
        div {
            class: classes!("dioxico-alert", state.get().then_some("dioxico-alert-open")),

            div {
                class: "dioxico-alert-inner",

                div {
                    class: "dioxico-alert-header",

                    div {
                        class: "dioxico-alert-header-space",
                    }

                    h4 {
                        class: "dioxico-alert-title",

                        {title}
                    }

                    IconButton {
                        icon: XMARK_ICON,
                        size: IconButtonSize::Medium,
                        on_click: move |_| {
                            on_close.call(true);
                            state.set(false);

                            if let Some(task) = timeout_state() {
                                task.cancel();
                                timeout_state.set(None);
                            }
                        }
                    }
                }

                div {
                    class: "dioxico-alert-body",

                    {children}
                }
            }
        }
    }
}
