use dioxico::{Button, Popover, PopoverPositionX, PopoverPositionY, PopoverType};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut state1 = use_signal(|| false);
    let mut state2 = use_signal(|| false);

    rsx! {
      Button { on_click: move || state1.set(true), "Open un-anchored popover" }
      Popover {
        state: state1,
        backdrop: true,
        position_x: PopoverPositionX::Right,
        position_y: PopoverPositionY::Top,
        position_offset_x: 12,
        position_offset_y: 12,
        p { "An un-anchored popover with no styling." }
      }
      div {
        onmouseenter: move |_| state2.set(true),
        onmouseleave: move |_| state2.set(false),
        Popover {
          state: state2,
          popover_type: PopoverType::Manual,
          anchored: true,
          anchor_content: "Hover here to see anchored popover",
          animate: false,
          position_y: PopoverPositionY::Bottom,
          p { "An anchored popover with no styling." }
        }
      }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
