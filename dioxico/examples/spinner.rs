use dioxico::{Spinner, SpinnerSize};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Spinner {
            size: SpinnerSize::Small,
            center: false,
        }
        Spinner {
            size: SpinnerSize::Medium,
            center: false,
        }
        Spinner {
            size: SpinnerSize::Large,
            center: false,
        }
        Spinner {
            size: SpinnerSize::Max,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
