use crate::hooks::ClickAwayRegistry;
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
pub fn ConfigProvider(#[props(default, into)] class: String, children: Element) -> Element {
    let theme_signal = use_signal(Theme::default);
    let theme = use_context_provider(|| theme_signal);
    let theme_styles = theme.read().root_style();

    let click_away_reg_signal = use_signal(ClickAwayRegistry::new);
    use_context_provider(|| click_away_reg_signal);

    rsx! {
        div {
            class,
            onclick: move |_| {
                click_away_reg_signal.read().trigger_all();
            },

            Stylesheet {
                href: STYLES
            }

            DynamicStyle {
                "{theme_styles}"
            }

            {children}
        }
    }
}
