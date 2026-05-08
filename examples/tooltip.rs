use dioxico::Tooltip;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            text: "Tooltip hover text",
            "Hover here to view the tooltip text"
        }
        Tooltip {
            text: "This should not show",
            disabled: true,
            "This tooltip is disabled, and should show nothing when hovered over"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
