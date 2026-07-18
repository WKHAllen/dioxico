use dioxico::Slider;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let int_state = use_signal(|| 3u8);
    let float_state = use_signal(|| 1.6f32);

    rsx! {
        Slider {
            state: int_state,
            min: 1,
            max: 9,
            step: 2,
            label: "Int slider label",
        }
        span {
            "Value: {int_state()}"
        }
        Slider {
            state: float_state,
            min: -10.0,
            max: 10.0,
            step: 0.1,
            label: "Float slider label",
        }
        span {
            "Value: {float_state()}"
        }
        Slider {
            state: int_state,
            min: 1,
            max: 17,
            label: "Disabled slider",
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
