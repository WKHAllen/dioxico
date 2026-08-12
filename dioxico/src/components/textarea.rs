//! Textarea components and utilities.

use super::{Error, ErrorSize};
use crate::classes;
use crate::css_repr::CssRepr;
use crate::element::ElementLike;
use crate::state::State;
use crate::util::*;
use dioxico_macros::CssRepr;
use dioxus::prelude::*;

/// Textarea resize options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
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

/// A textarea element.
#[component]
pub fn Textarea(
    /// The textarea input state.
    #[props(into)]
    state: State<String>,
    /// Textarea label element.
    #[props(default, into)]
    label: ElementLike,
    /// Placeholder text for when the textarea is empty.
    #[props(default)]
    placeholder: String,
    /// Maximum number of characters allowed.
    #[props(default = 524_288)]
    max_length: usize,
    /// Number of rows of text within the box.
    #[props(default = 3)]
    rows: usize,
    /// Resizing options for the textarea.
    #[props(default)]
    resize: TextareaResize,
    /// Callback called when the textarea element is mounted.
    #[props(default)]
    on_mounted: EventHandler<Event<MountedData>>,
    /// Is this field required?
    #[props(default)]
    required: bool,
    /// Is this field disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If missing or empty, no error will be shown.
    #[props(default)]
    error: String,
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    let id = use_id();
    let invalid = !error.is_empty();

    rsx! {
      div {
        class: classes!(
            "dioxico-textarea-container", disabled
            .then_some("dioxico-textarea-container-disabled"), class
        ),

        div { class: "dioxico-textarea-label-container",
          label { class: "dioxico-textarea-label", r#for: id,
            {label}

            span { class: "dioxico-required-mark",
              if required {
                " *"
              }
            }
          }
        }

        div { class: "dioxico-textarea-box-container",
          textarea {
            class: classes!(
                "dioxico-textarea", format!("dioxico-textarea-resize-{}", resize.css_repr()),
                invalid.then_some("dioxico-textarea-invalid")
            ),
            id,
            value: "{state.get()}",
            oninput: move |event| state.set(event.value()),
            onmounted: on_mounted,
            rows,
            placeholder,
            maxlength: max_length,
            required,
            disabled,
          }
        }

        Error { message: error, size: ErrorSize::Small }
      }
    }
}
