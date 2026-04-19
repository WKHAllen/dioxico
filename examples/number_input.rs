use dioxico::{NumberInput, NumberState};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let int_state = use_signal(|| NumberState::new(3u16).min(0).max(100));
    let mut submitted_times = use_signal(|| 0usize);
    let int_error = if *int_state() == 3 {
        "How about something other than 3"
    } else {
        ""
    };
    let float_state = use_signal(|| NumberState::new(1.618f64).min(-5.0).max(5.0).decimals(5));
    let float_error = if *float_state() == 1.618 {
        "No phi, please"
    } else {
        ""
    };

    rsx! {
        NumberInput {
            state: int_state,
            label: "Int number input",
            placeholder: "Placeholder!",
            on_submit: move |_| {
                submitted_times += 1;
            },
            required: true,
            error: int_error,
        }
        span {
            "Value: {*int_state()}"
        }
        span {
            "Submitted {submitted_times} times"
        }
        NumberInput {
            state: float_state,
            label: "Float number input",
            placeholder: "Placeholder!",
            required: true,
            error: float_error,
        }
        span {
            "Value: {*float_state()}"
        }
        NumberInput {
            state: int_state,
            label: "Disabled number input",
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
