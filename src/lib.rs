//! # Dioxico
//!
//! A collection of UI components for Dioxus.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod classes;
mod components;
mod hooks;
mod state;
mod style;
mod theme;
mod util;

pub use components::*;
pub use hooks::*;
pub use state::{ReadState, State};
pub use theme::{ColorMode, Theme};
