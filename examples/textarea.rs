use dioxico::{Textarea, TextareaResize};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state1 = use_signal(|| "Textarea value".to_owned());
    let error = if state1.is_empty() {
        "Please enter a value".to_owned()
    } else {
        String::new()
    };
    let state2 = use_signal(|| "Initial value".to_owned());
    let state3 = use_signal(|| "Initial value".to_owned());

    rsx! {
        Textarea {
            state: state1,
            label: "Signal textarea label",
            placeholder: "Placeholder!",
            required: true,
            error: error,
        }
        span {
            "Value: {state1()}"
        }
        Textarea {
            state: state1,
            label: "Disabled textarea",
            disabled: true,
        }
        Textarea {
            state: state2,
            label: "Vertical resize",
            resize: TextareaResize::Vertical,
        }
        Textarea {
            state: state3,
            label: "Full resize",
            resize: TextareaResize::Both,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
