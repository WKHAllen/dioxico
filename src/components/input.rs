use super::{Error, ErrorSize};
use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;

/// The type of input element.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputType {
    /// Standard text input.
    #[default]
    Text,
    /// Email address input.
    Email,
    /// Telephone number input.
    Tel,
    /// URL input.
    Url,
    /// Password input.
    Password,
}

impl InputType {
    /// Gets the HTML input element type corresponding to the current input type.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Password => "password",
        }
    }
}

/// An input element.
#[component]
pub fn Input(
    #[props(into)] state: State<String>,
    #[props(default)] input_type: InputType,
    #[props(default)] label: String,
    #[props(default)] placeholder: String,
    #[props(default = 524288)] max_length: usize,
    #[props(default)] on_submit: EventHandler<()>,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] error: String,
) -> Element {
    let id = use_id();
    let invalid = !error.is_empty();

    rsx! {
        div {
            class: classes!("dioxico-input-container", disabled.then_some("dioxico-input-container-disabled")),

            label {
                class: "dioxico-input-label",
                r#for: "{id}",

                "{label}"

                span {
                    class: "dioxico-required-mark",

                    if required {
                        " *"
                    }
                }
            }

            div {
                class: "dioxico-input-box-container",

                input {
                    r#type: input_type.as_str(),
                    class: classes!("dioxico-input", invalid.then_some("dioxico-input-invalid")),
                    id,
                    value: "{state.get()}",
                    oninput: move |event| state.set(event.value()),
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            on_submit.call(());
                        }
                    },
                    placeholder,
                    maxlength: max_length,
                    required,
                    disabled,
                }
            }

            Error {
                message: error,
                size: ErrorSize::Small,
            }
        }
    }
}
