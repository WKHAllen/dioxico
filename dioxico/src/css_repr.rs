//! Utilities for representing values in CSS classes.

/// A CSS-representable enum.
pub trait CssRepr {
    /// Returns a representation of the currently active enum variant that can
    /// be used in a CSS class.
    fn css_repr(&self) -> &'static str; // Please stabilize `const fn` in traits. Please.
}
