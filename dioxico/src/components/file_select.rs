//! File selection components and utilities.

use super::ButtonStyle;
use crate::classes::*;
use crate::collection::Collection;
use crate::css_repr::CssRepr;
use crate::util::*;
pub use dioxus::html::FileData;
use dioxus::html::HasFileData;
use dioxus::prelude::*;

/// The style of a file select button.
pub type FileSelectButtonStyle = ButtonStyle;

/// File selection component.
#[component]
pub fn FileSelect(
    /// Button style.
    #[props(default)]
    style: FileSelectButtonStyle,
    /// Should directory selection be allowed?
    #[props(default)]
    directory: bool,
    /// Should selection of multiple files be allowed?
    #[props(default)]
    multiple: bool,
    /// List of acceptable file type specifiers. If empty, all files will be
    /// allowed.
    #[props(default, into)]
    accept: Collection<String>,
    /// Is this field disabled?
    #[props(default)]
    disabled: bool,
    /// CSS classes to apply to the file selection element.
    #[props(default, into)]
    class: String,
    /// Callback called when files or directories are selected.
    #[props(default)]
    on_select: EventHandler<Vec<FileData>>,
    /// Button inner elements.
    children: Element,
) -> Element {
    let id = use_id();
    let accept = if accept.is_empty() {
        "*".to_owned()
    } else {
        accept.join(",")
    };

    rsx! {
      div { class: classes!("dioxico-file-select", class),

        button {
          r#type: "button",
          class: classes!(
              "dioxico-file-select-button", format!("dioxico-file-select-button-{}", style
              .css_repr())
          ),
          disabled,

          label {
            r#for: "{id}",
            class: "dioxico-file-select-button-label",

            {children}
          }
        }

        input {
          r#type: "file",
          class: "dioxico-file-select-input",
          id,
          directory,
          multiple,
          accept,
          disabled,
          onchange: move |event| on_select.call(event.files()),
        }
      }
    }
}

/// File drop zone component.
#[component]
pub fn FileDrop(
    /// Should directory selection be allowed?
    #[props(default)]
    directory: bool,
    /// Should selection of multiple files be allowed?
    #[props(default)]
    multiple: bool,
    /// List of acceptable file type specifiers. If empty, all files will be
    /// allowed.
    #[props(default, into)]
    accept: Collection<String>,
    /// Is this field disabled?
    #[props(default)]
    disabled: bool,
    /// CSS classes to apply to the file drop element.
    #[props(default, into)]
    class: String,
    /// Callback called when files or directories are selected.
    #[props(default)]
    on_drop: EventHandler<Vec<FileData>>,
    /// Drop zone inner elements.
    children: Element,
) -> Element {
    let id = use_id();
    let accept = if accept.is_empty() {
        "*".to_owned()
    } else {
        accept.join(",")
    };
    let mut hovering = use_signal(|| false);

    rsx! {
      div {
        class: classes!(
            "dioxico-file-drop", hovering().then_some("dioxico-file-drop-dropping"), disabled
            .then_some("dioxico-file-drop-disabled"), class
        ),
        ondragover: move |event| {
            event.prevent_default();

            if !disabled {
                hovering.set(true);
            }
        },
        ondragleave: move |_| {
            if !disabled {
                hovering.set(false);
            }
        },
        ondrop: move |event| {
            event.prevent_default();

            if !disabled {
                hovering.set(false);
                on_drop.call(event.files());
            }
        },

        label { r#for: "{id}", class: "dioxico-file-drop-label",

          span { class: "dioxico-file-drop-label-text", {children} }
        }

        input {
          r#type: "file",
          class: "dioxico-file-drop-input",
          id,
          directory,
          multiple,
          accept,
          disabled,
          onchange: move |event| on_drop.call(event.files()),
        }
      }
    }
}
