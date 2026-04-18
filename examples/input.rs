use dioxico::Input;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Demo() -> Element {
    let state1 = use_signal(|| "Input value".to_owned());
    let error = if state1.is_empty() {
        "Please enter a value".to_owned()
    } else {
        String::new()
    };
    let state2 = "Initial value";
    let mut state3 = use_signal(|| "Initial value".to_owned());
    let mut state4 = use_signal(|| "Initial value".to_owned());

    rsx! {
        Input {
            state: state1,
            label: "Signal input label",
            placeholder: "Placeholder!",
            required: true,
            error: error,
        }
        span {
            "Value: "
            "{state1()}"
        }
        Input {
            state: state1,
            label: "Disabled input",
            disabled: true,
        }
        Input {
            state: state2,
            label: "Value input label",
        }
        span {
            "Value: "
            "{state2}"
        }
        Input {
            state: (state3(), move |value| state3.set(value)),
            label: "Value/callback input label",
        }
        span {
            "Value: "
            "{state3()}"
        }
        Input {
            state: (state4(), move |value| async move { state4.set(value) }),
            label: "Value/async callback input label",
        }
        span {
            "Value: "
            "{state4()}"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
