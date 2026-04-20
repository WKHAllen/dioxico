use dioxico::{IconButton, IconButtonSize, XMARK_ICON};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut times_clicked = use_signal(|| 0usize);

    rsx! {
        IconButton {
            icon: XMARK_ICON,
            size: IconButtonSize::Small,
            on_click: move |_| times_clicked += 1,
        }
        IconButton {
            icon: XMARK_ICON,
            size: IconButtonSize::Medium,
            on_click: move |_| times_clicked += 1,
        }
        IconButton {
            icon: XMARK_ICON,
            size: IconButtonSize::Large,
            on_click: move |_| times_clicked += 1,
        }
        span {
            "Icon button has been clicked {times_clicked()} times"
        }
        IconButton {
            icon: XMARK_ICON,
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
