//! Utilities involving element types.

use dioxus::core::{DynamicNode, SuperFrom};
use dioxus::prelude::*;
use std::rc::Rc;

/// Anything that can be rendered, e.g. behaves like an element.
#[derive(Default, Clone)]
pub struct ElementLike {
    /// A function that returns the provided element into a `DynamicNode` via
    /// the `IntoDynNode` trait.
    f: Option<Rc<dyn Fn() -> DynamicNode>>,
}

impl ElementLike {
    /// Is this node empty?
    pub const fn is_empty(&self) -> bool {
        self.f.is_none()
    }
}

impl PartialEq for ElementLike {
    fn eq(&self, other: &Self) -> bool {
        match (&self.f, &other.f) {
            (Some(f1), Some(f2)) => Rc::ptr_eq(f1, f2),
            (None, None) => true,
            _ => false,
        }
    }
}

/// Marker type for `SuperFrom`.
pub struct DynNodeMarker;

impl<T> SuperFrom<T, DynNodeMarker> for ElementLike
where
    T: IntoDynNode + Clone + 'static,
{
    fn super_from(value: T) -> Self {
        Self {
            f: Some(Rc::new(move || value.clone().into_dyn_node())),
        }
    }
}

impl From<String> for ElementLike {
    fn from(value: String) -> Self {
        SuperFrom::<String, DynNodeMarker>::super_from(value)
    }
}

impl IntoDynNode for ElementLike {
    fn into_dyn_node(self) -> dioxus_core::DynamicNode {
        match self.f {
            Some(f) => f(),
            None => ().into_dyn_node(),
        }
    }
}
