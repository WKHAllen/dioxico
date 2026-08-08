use dioxico::{Button, Popover, PopoverPositionX, PopoverPositionY, PopoverType, use_popover};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let handle1 = use_popover();
    let handle2 = use_popover();

    rsx! {
      Button { on_click: move || handle1.open(), "Open un-anchored popover" }
      Popover {
        handle: handle1,
        backdrop: true,
        position_x: PopoverPositionX::Right,
        position_y: PopoverPositionY::Top,
        position_offset_x: 12,
        position_offset_y: 12,
        p { "An un-anchored popover with no styling." }
      }
      div {
        onmouseenter: move |_| handle2.open(),
        onmouseleave: move |_| handle2.close(),
        Popover {
          handle: handle2,
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
