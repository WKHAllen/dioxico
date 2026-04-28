use dioxico::{Alert, Button};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut alert_finite_int_state = use_signal(|| false);
    let mut alert_finite_float_state = use_signal(|| false);
    let mut alert_infinite_state = use_signal(|| false);
    let mut alert_close_state = use_signal(|| None);

    rsx! {
        Button {
            text: "Open 5 second alert",
            on_click: move |_| alert_finite_int_state.set(true),
        }
        Alert {
            state: alert_finite_int_state,
            title: "Finite 5 second alert",
            duration: 5u64,
            on_close: move |manual| alert_close_state.set(Some(manual)),

            p { "This alert will only remain open for 5 seconds." }
        }
        Button {
            text: "Open 6.789 second alert",
            on_click: move |_| alert_finite_float_state.set(true),
        }
        Alert {
            state: alert_finite_float_state,
            title: "Finite 6.789 second alert",
            duration: 6.789f64,
            on_close: move |manual| alert_close_state.set(Some(manual)),

            p { "This alert will only remain open for 6.789 seconds." }
        }
        Button {
            text: "Open infinite alert",
            on_click: move |_| alert_infinite_state.set(true),
        }
        Alert {
            state: alert_infinite_state,
            title: "Infinite alert",
            on_close: move |manual| alert_close_state.set(Some(manual)),

            p { "This alert will only remain open until the 'x' button is pressed." }
        }
        span {
            "Close value: {alert_close_state():?}"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
