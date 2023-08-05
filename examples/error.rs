use dioxico::{Error, ErrorSize};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Demo(cx: Scope) -> Element {
    cx.render(rsx! {
        Error {
            message: Some("The smallest error message"),
            size: ErrorSize::Smaller
        }
        Error {
            message: Some("The small error message"),
            size: ErrorSize::Small
        }
        Error {
            message: Some("The medium size error message"),
            size: ErrorSize::Medium
        }
        Error {
            message: Some("The large error message"),
            size: ErrorSize::Large
        }
        Error {
            message: Some("The largest error message"),
            size: ErrorSize::Larger
        }
    })
}

#[allow(dead_code)]
fn main() {
    dioxus_web::launch(Demo);
}
