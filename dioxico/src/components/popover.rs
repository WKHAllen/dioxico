//! Popover utilities.

use crate::CssRepr;
use crate::classes;
use crate::element::ElementLike;
use crate::state::State;
use crate::util::*;
use dioxus::core::anyhow;
use dioxus::prelude::*;

/// Opens an HTML popover element.
fn open_popover(id: &str) {
    document::eval(&format!("document.getElementById('{id}').showPopover();"));
}

/// Closes an HTML popover element.
fn close_popover(id: &str) {
    document::eval(&format!("document.getElementById('{id}').hidePopover();"));
}

/// Checks whether the popover with the given ID is open.
#[allow(clippy::future_not_send)]
async fn is_popover_open(id: &str) -> Result<bool> {
    let eval = document::eval(&format!(
        "return document.getElementById('{id}').matches(':popover-open');"
    ));
    let res = eval.await?;
    res.as_bool()
        .ok_or_else(|| anyhow!("`is_popover_open` return value was not a boolean").into())
}

/// Popover type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum PopoverType {
    /// Automatic popover type.
    #[default]
    Auto,
    /// Hint popover type.
    Hint,
    /// Manual popover type.
    Manual,
}

/// Popover X-axis position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum PopoverPositionX {
    /// Positioned left of the anchor.
    Left,
    /// Positioned central to the anchor.
    Center,
    /// Positioned right of the anchor.
    Right,
}

/// Popover Y-axis position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum PopoverPositionY {
    /// Positioned on top of the anchor.
    Top,
    /// Positioned central to the anchor.
    Center,
    /// Positioned on the bottom of the anchor.
    Bottom,
}

/// A generalized popover component.
#[component]
pub fn Popover(
    /// The popover state.
    #[props(into)]
    state: State<bool>,
    /// Popover type.
    #[props(default)]
    popover_type: PopoverType,
    /// If set to `true`, an anchor element will be created. If `false` or not
    /// specified, no anchor element will be created, and the popover will be
    /// anchored to the document body.
    #[props(default = false)]
    anchored: bool,
    /// A CSS class for the anchor element.
    #[props(default, into)]
    anchor_class: String,
    /// Content within the anchor element.
    #[props(default, into)]
    anchor_content: ElementLike,
    /// Should there be a backdrop overlay when the popover is open?
    #[props(default = false)]
    backdrop: bool,
    /// Should the transitions between open/closed state animate? Animations can
    /// be implemented manually if needed.
    #[props(default = true)]
    animate: bool,
    /// X-axis positioning of the popover relative to the anchor.
    position_x: Option<PopoverPositionX>,
    /// Y-axis positioning of the popover relative to the anchor.
    position_y: Option<PopoverPositionY>,
    /// Number of pixels to offset the popover on the X-axis.
    #[props(default)]
    position_offset_x: i32,
    /// Number of pixels to offset the popover on the Y-axis.
    #[props(default)]
    position_offset_y: i32,
    /// Callback called when the popover is dismissed.
    #[props(default)]
    on_dismiss: EventHandler<()>,
    /// Should the popover close when dismissed? Defaults to `true`.
    #[props(default = true)]
    close_on_dismiss: bool,
    /// CSS classes to apply to the popover element.
    #[props(default, into)]
    class: String,
    /// Elements within the popover.
    children: Element,
) -> Element {
    let id = use_id();

    use_effect({
        let id = id.clone();
        move || {
            if state.get() {
                open_popover(&id);
            } else {
                close_popover(&id);
            }
        }
    });

    let pos_x = position_x.map(|pos| pos.css_repr()).unwrap_or_default();
    let pos_y = position_y.map(|pos| pos.css_repr()).unwrap_or_default();
    let position_area_style = if pos_x.is_empty() && pos_y.is_empty() {
        String::new()
    } else {
        format!("position-area: {pos_x} {pos_y};")
    };
    let anchor_style = if anchored {
        format!("position-anchor: --anchor-{id};")
    } else {
        String::new()
    };
    let offset_x_style = if let Some(pos) = position_x
        && pos != PopoverPositionX::Center
    {
        format!("margin-{}: {}px;", pos.css_repr(), position_offset_x)
    } else {
        String::new()
    };
    let offset_y_style = if let Some(pos) = position_y
        && pos != PopoverPositionY::Center
    {
        format!("margin-{}: {}px;", pos.css_repr(), position_offset_y)
    } else {
        String::new()
    };

    rsx! {
      if anchored {
        div {
          class: classes!("dioxico-popover-anchor", anchor_class),
          style: "anchor-name: --anchor-{id};",
          {anchor_content}
        }
      }

      div {
        class: classes!(
            "dioxico-popover", backdrop.then_some("dioxico-popover-backdrop"), animate
            .then_some("dioxico-popover-animate"), position_x.map(| pos |
            format!("dioxico-popover-{}", pos.css_repr())), position_y.map(| pos |
            format!("dioxico-popover-{}", pos.css_repr())), class
        ),
        id: "{id}",
        ontoggle: move |_| {
            let id = id.clone();
            async move {
                let open = is_popover_open(&id).await.unwrap();

                if !open && state.get() {
                    if close_on_dismiss {
                        state.set(false);
                    } else {
                        open_popover(&id);
                    }

                    on_dismiss.call(());
                }
            }
        },
        popover: popover_type.css_repr(),
        style: "{anchor_style} {position_area_style} {offset_x_style} {offset_y_style}",
        {children}
      }
    }
}
