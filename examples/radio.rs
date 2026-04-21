use dioxico::{RadioGroup, RadioGroupOrientation};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state = use_signal(|| None);
    let options = (0..3)
        .map(|index| format!("Option {}", index + 1))
        .collect::<Vec<_>>();

    rsx! {
        RadioGroup {
            state,
            options: options.clone(),
        }
        span {
            "Value: {state():?}"
        }
        RadioGroup {
            state,
            options,
            orientation: RadioGroupOrientation::Horizontal,
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
