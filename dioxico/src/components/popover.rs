//! Popover utilities.

use crate::CssRepr;
use crate::ElementLike;
use crate::classes;
use crate::util::*;
use dioxus::prelude::*;

/// Opens an HTML popover element.
fn open_popover(id: &str) {
    document::eval(&format!("document.getElementById('{id}').showPopover();"));
}

/// Closes an HTML popover element.
fn close_popover(id: &str) {
    document::eval(&format!("document.getElementById('{id}').hidePopover();"));
}

/// Toggles an HTML popover element.
fn toggle_popover(id: &str) {
    document::eval(&format!("document.getElementById('{id}').togglePopover();"));
}

/// A handle to a popover element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PopoverHandle {
    /// The ID of the popover element.
    id: [u8; 8],
}

impl PopoverHandle {
    /// Returns the ID of the popover element.
    #[allow(clippy::missing_panics_doc)] // cannot panic while invariants are upheld
    #[inline]
    pub fn id(&self) -> &str {
        str::from_utf8(&self.id).unwrap()
    }

    /// Opens this popover.
    #[inline]
    pub fn open(self) {
        open_popover(self.id());
    }

    /// Closes this popover.
    #[inline]
    pub fn close(self) {
        close_popover(self.id());
    }

    /// Toggles this popover.
    #[inline]
    pub fn toggle(self) {
        toggle_popover(self.id());
    }
}

/// Creates a handle to a popover element.
///
/// **Note:** In order to use this, the popover must be created in the DOM as an
/// HTML element with the `popover` attribute, and with the ID provded by
/// [`PopoverHandle::id()`].
#[allow(clippy::missing_panics_doc)] // cannot panic while invariants are upheld
pub fn use_popover() -> PopoverHandle {
    use_hook(|| PopoverHandle {
        id: new_id().bytes().collect::<Vec<_>>().try_into().unwrap(),
    })
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
    /// A handle to this popover. Use [`use_popover()`] to create a handle.
    handle: PopoverHandle,
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
    /// CSS classes to apply to the popover element.
    #[props(default, into)]
    class: String,
    /// Elements within the popover.
    children: Element,
) -> Element {
    let pos_x = position_x.map(|pos| pos.css_repr()).unwrap_or_default();
    let pos_y = position_y.map(|pos| pos.css_repr()).unwrap_or_default();
    let position_area_style = if pos_x.is_empty() && pos_y.is_empty() {
        String::new()
    } else {
        format!("position-area: {pos_x} {pos_y};")
    };
    let anchor_style = if anchored {
        format!("position-anchor: --anchor-{};", handle.id())
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
          style: "anchor-name: --anchor-{handle.id()};",
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
        id: handle.id(),
        popover: popover_type.css_repr(),
        style: "{anchor_style} {position_area_style} {offset_x_style} {offset_y_style}",
        {children}
      }
    }
}
