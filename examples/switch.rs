use dioxico::Switch;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state = use_signal(|| true);

    rsx! {
        Switch {
            state,
            label: "Switch label",
        }
        span {
            "Value: {state()}"
        }
        Switch {
            state,
            label: "Disabled switch",
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
