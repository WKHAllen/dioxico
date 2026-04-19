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
    /// The input state.
    #[props(into)]
    state: State<String>,
    /// Input type.
    #[props(default)]
    input_type: InputType,
    /// Input label text.
    #[props(default)]
    label: String,
    /// Placeholder text for when the input box is empty.
    #[props(default)]
    placeholder: String,
    /// Maximum number of characters allowed.
    #[props(default = 524288)]
    max_length: usize,
    /// Callback called when the enter key is pressed.
    #[props(default)]
    on_submit: EventHandler<()>,
    /// Is this field required?
    #[props(default)]
    required: bool,
    /// Is this field disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If missing or empty, no error will be shown.
    #[props(default)]
    error: String,
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
