use dioxico::{Badge, BadgeStyle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Badge {
            value: 3,
            style: BadgeStyle::Primary,
        }
        Badge {
            value: 42,
            style: BadgeStyle::Secondary,
        }
        Badge {
            value: 1.618,
            style: BadgeStyle::Danger,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
