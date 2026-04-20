use dioxico::{Icon, IconSize, XMARK_ICON};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Icon {
            icon: XMARK_ICON,
            size: IconSize::Small,
        }
        Icon {
            icon: XMARK_ICON,
            size: IconSize::Medium,
        }
        Icon {
            icon: XMARK_ICON,
            size: IconSize::Large,
        }
        Icon {
            icon: XMARK_ICON,
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
