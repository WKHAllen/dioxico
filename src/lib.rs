//! # Dioxico
//!
//! A collection of UI components for Dioxus.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![allow(non_snake_case)]

mod classes;
mod components;
mod state;
mod theme;
mod util;

pub use components::*;
pub use state::{ReadState, State};
pub use theme::{use_theme, ColorMode, Theme};
