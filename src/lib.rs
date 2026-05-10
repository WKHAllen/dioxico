//! # Dioxico
//!
//! A collection of UI components for Dioxus.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod classes;
mod collection;
mod components;
mod element;
mod hooks;
mod state;
mod style;
mod theme;
mod util;

pub use collection::Collection;
pub use components::*;
pub use element::ElementLike;
pub use hooks::*;
pub use state::State;
pub use theme::{ColorMode, Theme};
