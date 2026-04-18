use super::error::*;
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
    #[props(default)] required: ReadSignal<bool>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] error: ReadSignal<String>,
) -> Element {
    let id = use_id();
    let html_input_type = input_type.as_str();
    let container_class = classes!(
        "dioxico-input-container",
        disabled().then_some("dioxico-input-container-disabled")
    );
    let input_class = classes!(
        "dioxico-input",
        (!error.read().is_empty()).then_some("dioxico-input-invalid")
    );

    rsx! {
        div {
            class: "{container_class}",

            label {
                class: "dioxico-input-label",
                r#for: "{id}",

                "{label}"

                span {
                    class: "dioxico-required-mark",

                    if required() {
                        " *"
                    }
                }
            }

            input {
                class: "{input_class}",
                value: "{state.get()}",
                oninput: move |event| state.set(event.value()),
                id: "{id}",
                r#type: "{html_input_type}",
                placeholder: "{placeholder}",
                maxlength: "{max_length}",
                required: "{required()}",
                disabled: "{disabled()}",
            }

            Error {
                message: error,
                size: ErrorSize::Small,
            }
        }
    }
}
