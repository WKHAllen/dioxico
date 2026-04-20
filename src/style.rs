use crate::util::*;
use dioxus::document::{document, StyleProps};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[component]
pub fn DynamicStyle(children: Element) -> Element {
    // This value has to be untracked (e.g. must use `Rc<RefCell<T>>` rather
    // than a signal) because the necessary state change below would otherwise
    // create an infinite loop.
    let old_el_id = use_hook(|| Rc::new(RefCell::new(None)));
    let new_el_id = new_id();
    let document = document();

    // Recreating the stylesheet each time is not a great solution, but
    // alternative solutions are equally bad. This solution will do for a small
    // enough set of styles.
    document.create_style(
        StyleProps::builder()
            .id(new_el_id.as_str())
            .children(children)
            .build(),
    );

    if let Some(old_id) = &*old_el_id.borrow() {
        document.eval(format!("document.getElementById('{}').remove();", old_id));
    }

    *old_el_id.borrow_mut() = Some(new_el_id);

    VNode::empty()
}
