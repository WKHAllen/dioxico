use dioxico::Input;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Demo() -> Element {
    let state = use_signal(|| "Input value".to_owned());
    let error = if state.is_empty() {
        "Please enter a value".to_owned()
    } else {
        String::new()
    };

    rsx! {
        Input {
            state: state,
            label: "Input label",
            placeholder: "Placeholder!",
            required: true,
            error: error,
        }
        span {
            "Value: "
            "{state()}"
        }
        Input {
            state: state,
            label: "Disabled input",
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
