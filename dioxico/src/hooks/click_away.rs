//! Hooks and hook-related utilities involving detecting when a user clicks away
//! from an element.

use dioxus::prelude::*;
use std::collections::HashMap;

/// A registry of functions to call when a click-away event is detected.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClickAwayRegistry {
    /// The callback function registry.
    callbacks: HashMap<usize, Callback>,
    /// The next available ID.
    next_id: usize,
}

impl ClickAwayRegistry {
    /// Constructs a new empty registry.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new callback and returns its ID.
    pub fn register(&mut self, cb: Callback) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.callbacks.insert(id, cb);
        id
    }

    /// Unregisters an existing callback, given its ID.
    pub fn unregister(&mut self, id: usize) {
        self.callbacks.remove(&id);
    }

    /// Runs all callbacks.
    pub fn trigger_all(&self) {
        for cb in self.callbacks.values() {
            cb.call(());
        }
    }

    /// Runs all callbacks except the one with the given ID.
    pub fn trigger_all_except(&self, id: usize) {
        for (cbid, cb) in &self.callbacks {
            if *cbid != id {
                cb.call(());
            }
        }
    }
}

/// Detects when an element has been clicked away from.
///
/// The provided function will be executed when that detection triggers. The
/// returned function must be registered as an `onclick` event listener for the
/// desired element.
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
