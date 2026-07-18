use dioxico::Checkbox;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state = use_signal(|| true);

    rsx! {
        Checkbox {
            state,
            label: "Checkbox label",
        }
        span {
            "Value: {state()}"
        }
        Checkbox {
            state,
            label: "Disabled checkbox",
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
