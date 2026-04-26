use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ClickAwayRegistry {
    callbacks: HashMap<usize, Callback>,
    next_id: usize,
}

impl ClickAwayRegistry {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, cb: Callback) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.callbacks.insert(id, cb);
        id
    }

    pub fn unregister(&mut self, id: usize) {
        self.callbacks.remove(&id);
    }

    pub fn trigger_all(&self) {
        for cb in self.callbacks.values() {
            cb.call(());
        }
    }

    pub fn trigger_all_except(&self, id: usize) {
        for (cbid, cb) in &self.callbacks {
            if *cbid != id {
                cb.call(());
            }
        }
    }
}

/// Detects when an element has been clicked away from. The provided function
/// will be executed when that detection triggers. The returned function must be
/// registered as an `onclick` event listener for the desired element.
///
/// ```
/// # use dioxico::use_click_away;
/// # use dioxus::prelude::*;
/// #
/// # fn Demo() -> Element {
/// let click_away_onclick = use_click_away(|| println!("clicked away from element!"));
///
/// rsx! {
///     span {
///         onclick: click_away_onclick,
///         "Click away from this element"
///     }
/// }
/// # }
/// ```
///
/// This must be used inside of a [`ConfigProvider`](crate::ConfigProvider).
pub fn use_click_away(mut f: impl FnMut() + 'static) -> impl FnMut(Event<MouseData>) {
    let mut registry = use_context::<Signal<ClickAwayRegistry>>();
    let cb = use_callback(move |_| f());

    let id = use_hook(|| registry.write().register(cb));

    use_drop(move || registry.write().unregister(id));

    move |event| {
        event.stop_propagation();
        registry.read().trigger_all_except(id);
    }
}
