use crate::style::DynamicStyle;
use crate::theme::{Theme, STYLES};
use dioxus::prelude::*;

/// Must be inserted at one of the highest levels of the DOM tree. Injects all
/// necessary styles into the application.
///
/// ```
/// # use dioxico::ConfigProvider;
/// # use dioxus::prelude::*;
/// #
/// # fn Demo() -> Element {
/// rsx! {
///     ConfigProvider {
///         main {
///             // content here...
///         }
///     }
/// }
/// # }
/// ```
#[component]
pub fn ConfigProvider(children: Element) -> Element {
    let theme_signal = use_signal(Theme::default);
    let theme = use_context_provider(|| theme_signal);
    let theme_styles = theme.read().root_style();

    rsx! {
        Stylesheet {
            href: STYLES
        }

        DynamicStyle {
            "{theme_styles}"
        }

        {children}
    }
}
