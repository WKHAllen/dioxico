//! # Dioxico
//!
//! A collection of UI components for Dioxus.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::if_not_else)]
#![allow(clippy::ignored_unit_patterns)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::option_if_let_else)]

mod classes;
mod collection;
mod components;
mod element;
mod hooks;
mod state;
mod style;
mod theme;
mod unit_enum;
mod util;

pub use collection::Collection;
pub use components::*;
pub use dioxico_macros::*;
pub use element::ElementLike;
pub use hooks::*;
pub use state::State;
pub use theme::{ColorMode, Theme};
pub use unit_enum::UnitEnum;
