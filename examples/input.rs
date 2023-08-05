use dioxico::Input;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Demo(cx: Scope) -> Element {
    let state = use_state(cx, || "Input value".to_owned());
    let error = state.is_empty().then_some("Please enter a value");

    cx.render(rsx! {
        Input {
            state: state,
            label: "Input label",
            placeholder: "Placeholder!",
            required: true,
            error: error
        }
        span {
            "Value: "
            state.as_str()
        }
        Input {
            state: state,
            label: "Disabled input",
            disabled: true
        }
    })
}

#[allow(dead_code)]
fn main() {
    dioxus_web::launch(Demo);
}
