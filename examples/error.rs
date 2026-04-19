use dioxico::{Error, ErrorSize};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Error {
            message: "The smallest error message",
            size: ErrorSize::Smaller
        }
        Error {
            message: "The small error message",
            size: ErrorSize::Small
        }
        Error {
            message: "The medium size error message",
            size: ErrorSize::Medium
        }
        Error {
            message: "The large error message",
            size: ErrorSize::Large
        }
        Error {
            message: "The largest error message",
            size: ErrorSize::Larger
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
