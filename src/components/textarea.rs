use super::{Error, ErrorSize};
use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;

/// Textarea resize options.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextareaResize {
    /// No resize.
    #[default]
    None,
    /// Horizontal resize only.
    Horizontal,
    /// Vertical resize only.
    Vertical,
    /// Both horizontal and vertical resize.
    Both,
}

impl TextareaResize {
    /// Gets the name of the resize option.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::None => "none",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
            Self::Both => "both",
        }
    }
}

/// A textarea element.
#[component]
pub fn Textarea(
    /// The textarea input state.
    #[props(into)]
    state: State<String>,
    /// Textarea label text.
    #[props(default)]
    label: String,
    /// Placeholder text for when the textarea is empty.
    #[props(default)]
    placeholder: String,
    /// Maximum number of characters allowed.
    #[props(default = 524288)]
    max_length: usize,
    /// Number of rows of text within the box.
    #[props(default = 3)]
    rows: usize,
    /// Resizing options for the textarea.
    #[props(default)]
    resize: TextareaResize,
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
            class: classes!("dioxico-textarea-container", disabled.then_some("dioxico-textarea-container-disabled")),

            div {
                class: "dioxico-textarea-label-container",

                label {
                    class: "dioxico-textarea-label",
                    r#for: "{id}",

                    "{label}"

                    span {
                        class: "dioxico-required-mark",

                        if required {
                            " *"
                        }
                    }
                }
            }

            div {
                class: "dioxico-textarea-box-container",

                textarea {
                    class: classes!("dioxico-textarea", format!("dioxico-textarea-resize-{}", resize.as_str()), invalid.then_some("dioxico-textarea-invalid")),
                    id,
                    value: "{state.get()}",
                    oninput: move |event| state.set(event.value()),
                    rows,
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
