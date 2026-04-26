use crate::theme::Theme;
use dioxus::prelude::*;

/// Applies a styling theme. The default theme will be used initially, but it
/// can be altered via the returned signal.
///
/// ```
/// # use dioxico::use_theme;
/// # use dioxus::prelude::*;
/// #
/// # fn Demo() -> Element {
/// let mut theme = use_theme();
/// theme.write().set_primary_color((105, 40, 255));
/// #
/// # rsx! { Fragment { } }
/// # }
/// ```
///
/// No styles will be loaded until this function is called. When the scope is
/// disposed, the styles will be removed. For these reasons, this should
/// probably be called immediately and at the highest level of the
/// application.
pub fn use_theme() -> Signal<Theme> {
    use_context()
}
