//! Utilities for select-able types.

/// A select-able enum.
///
/// This trait can be derived on any enum that contains only unit variants. Any
/// such enum that implements this trait can then be used in a `SelectEnum`.
pub trait UnitEnum: Sized {
    /// A static list of the names of all variants of this enum.
    const VARIANT_NAMES: &[&'static str];

    /// Returns the current variant's name.
    fn variant_name(&self) -> &'static str;

    /// Creates an instance of this type given the variant name. Returns `None`
    /// if the variant name is invalid.
    fn from_variant_name(name: &str) -> Option<Self>;
}
