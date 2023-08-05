use super::error::*;
use crate::classes::*;
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
    pub fn html_input_type(&self) -> &'static str {
        match *self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Password => "password",
        }
    }
}

/// Input properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// The input state.
    state: &'a UseState<String>,
    /// The input type.
    #[props(default)]
    input_type: InputType,
    /// The input label.
    label: Option<&'a str>,
    /// Input placeholder text.
    placeholder: Option<&'a str>,
    /// The maximum number of characters allowed.
    #[props(default = 524288)]
    max_length: usize,
    /// Whether the input is required to be filled out.
    #[props(default = false)]
    required: bool,
    /// Whether the input is disabled.
    #[props(default = false)]
    disabled: bool,
    /// An optional error message.
    #[props(!optional, default)]
    error: Option<&'a str>,
}

/// An input element.
pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let id = use_state(cx, new_id);
    let html_input_type = cx.props.input_type.html_input_type();
    let label = cx.props.label.unwrap_or_default();
    let placeholder = cx.props.placeholder.unwrap_or_default();
    let max_length = cx.props.max_length;
    let container_class = classes!(
        "dioxico-input-container",
        cx.props
            .disabled
            .then_some("dioxico-input-container-disabled")
    );
    let input_class = classes!(
        "dioxico-input",
        cx.props.error.map(|_| "dioxico-input-invalid")
    );

    cx.render(rsx! {
        div {
            class: "{container_class}",

            label {
                class: "dioxico-input-label",

                r#for: "{id}",

                "{label}"

                span {
                    class: "dioxico-required-mark",

                    cx.props.required.then(|| rsx! { " *" })
                }
            }

            input {
                class: "{input_class}",

                value: "{cx.props.state}",
                oninput: move |e| cx.props.state.set(e.value.clone()),
                id: "{id}",
                r#type: "{html_input_type}",
                placeholder: "{placeholder}",
                maxlength: "{max_length}",
                required: "{cx.props.required}",
                disabled: "{cx.props.disabled}"
            }

            Error {
                message: cx.props.error,
                size: ErrorSize::Small
            }
        }
    })
}
