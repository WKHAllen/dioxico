use super::{Error, ErrorSize, Icon, ANGLE_DOWN_ICON};
use crate::classes::*;
use crate::hooks::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;
use std::fmt::Display;

/// Helper trait to note which types can be used to track the state of a select
/// component.
pub trait SelectState: Copy {
    /// Does this select state type support null options?
    const HAS_NULL_OPTION: bool;

    /// Returns the selection state.
    fn get_value(&self) -> Option<usize>;

    /// Sets the selection state.
    fn set_value(&mut self, value: usize);

    /// Sets the selection to the null value, if applicable.
    fn set_null_value(&mut self);
}

impl SelectState for usize {
    const HAS_NULL_OPTION: bool = false;

    fn get_value(&self) -> Option<usize> {
        Some(*self)
    }

    fn set_value(&mut self, value: usize) {
        *self = value;
    }

    fn set_null_value(&mut self) {
        unimplemented!("no null value for `usize`")
    }
}

impl SelectState for Option<usize> {
    const HAS_NULL_OPTION: bool = true;

    fn get_value(&self) -> Option<usize> {
        *self
    }

    fn set_value(&mut self, value: usize) {
        *self = Some(value);
    }

    fn set_null_value(&mut self) {
        *self = None;
    }
}

/// Position of a select popup.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelectPopupPosition {
    /// Position the popup above.
    Above,
    /// Position the popup below.
    #[default]
    Below,
}

impl SelectPopupPosition {
    /// Gets the name of the position.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Above => "above",
            Self::Below => "below",
        }
    }
}

/// Select dropdown component.
#[component]
fn SelectInner<S, T>(
    /// Selection state.
    #[props(into)]
    state: State<S>,
    /// List of select options.
    #[props(into)]
    options: Vec<T>,
    /// Null label option text. Defaults to "Select...".
    #[props(default = "Select...".to_owned())]
    null_label: String,
    /// Select label.
    #[props(default)]
    label: String,
    /// Positioning of the popup.
    #[props(default)]
    position: SelectPopupPosition,
    /// Is this select field required?
    #[props(default)]
    required: bool,
    /// Is this select field disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If empty or missing, no error will be shown.
    #[props(default)]
    error: String,
) -> Element
where
    S: SelectState + Clone + PartialEq + 'static,
    T: Display + Clone + PartialEq + 'static,
{
    let id = use_id();
    let invalid = !error.is_empty();
    let mut dropdown_open = use_signal(|| false);

    // TODO: enable selecting via arrow keys, space/enter, and escape/backspace
    // let mut selecting = use_signal(|| None);

    // use_effect(move || {
    //     if !dropdown_open() {
    //         selecting.set(None);
    //     }
    // });

    let selected_option = match state.get().get_value() {
        Some(selected) => options
            .get(selected)
            .map(ToString::to_string)
            .unwrap_or_else({
                let null_label = null_label.clone();
                move || null_label
            }),
        None => null_label.clone(),
    };

    let select_node_onclick = use_click_away(move || dropdown_open.set(false));

    // TODO: `use_popup`?
    // let mut popup_node = use_signal(|| None);

    rsx! {
        div {
            class: classes!("dioxico-select-container", disabled.then_some("dioxico-select-container-disabled"), dropdown_open().then_some("dioxico-select-container-open")),

            div {
                class: "dioxico-select-label-container",

                label {
                    r#for: "{id}",
                    class: "dioxico-select-label",

                    {label}

                    span {
                        class: "dioxico-required-mark",

                        if required {
                            " *"
                        }
                    }
                }
            }

            div {
                class: classes!("dioxico-select", format!("dioxico-select-{}", position.as_str())),
                onclick: select_node_onclick,

                button {
                    r#type: "button",
                    class: classes!("dioxico-select-button", invalid.then_some("dioxico-select-button-invalid")),
                    id,
                    disabled,
                    onclick: move |_| {
                        if !disabled {
                            dropdown_open.set(!dropdown_open());
                        }
                    },

                    div {
                        class: "dioxico-select-button-selection",

                        {selected_option}
                    }

                    Icon {
                        icon: ANGLE_DOWN_ICON,
                        disabled,
                        class: "dioxico-select-button-icon",
                    }
                }

                div {
                    class: "dioxico-select-dropdown",

                    div {
                        class: "dioxico-select-popup",
                        // onmounted: move |element| {
                        //     popup_node.set(Some(element.data()));
                        // },

                        if S::HAS_NULL_OPTION {
                            div {
                                class: "dioxico-select-option",
                                onclick: move |_| {
                                    state.with_mut(|state| state.set_null_value());
                                    dropdown_open.set(false);
                                },

                                {null_label}
                            }
                        }

                        for (index, option) in options.into_iter().enumerate() {
                            div {
                                class: "dioxico-select-option",
                                onclick: move |_| {
                                    state.with_mut(|state| state.set_value(index));
                                    dropdown_open.set(false);
                                },

                                {option.to_string()}
                            }
                        }
                    }
                }
            }

            Error {
                message: error,
                size: ErrorSize::Small,
                class: "dioxico-select-error",
            }
        }
    }
}

/// Select dropdown component.
#[component]
pub fn Select<T>(
    /// Selection state.
    #[props(into)]
    state: State<usize>,
    /// List of select options.
    #[props(into)]
    options: Vec<T>,
    /// Select label.
    #[props(default)]
    label: String,
    /// Positioning of the popup.
    #[props(default)]
    position: SelectPopupPosition,
    /// Is this select field required?
    #[props(default)]
    required: bool,
    /// Is this select field disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If empty or missing, no error will be shown.
    #[props(default)]
    error: String,
) -> Element
where
    T: Display + Clone + PartialEq + 'static,
{
    rsx! {
        SelectInner<usize, T> {
            state,
            options,
            label,
            position,
            required,
            disabled,
            error,
        }
    }
}

/// Select dropdown component.
#[component]
pub fn SelectNullable<T>(
    /// Selection state.
    #[props(into)]
    state: State<Option<usize>>,
    /// List of select options.
    #[props(into)]
    options: Vec<T>,
    /// Null label option text. Defaults to "Select...".
    #[props(default = "Select...".to_owned())]
    null_label: String,
    /// Select label.
    #[props(default)]
    label: String,
    /// Positioning of the popup.
    #[props(default)]
    position: SelectPopupPosition,
    /// Is this select field required?
    #[props(default)]
    required: bool,
    /// Is this select field disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If empty or missing, no error will be shown.
    #[props(default)]
    error: String,
) -> Element
where
    T: Display + Clone + PartialEq + 'static,
{
    rsx! {
        SelectInner<Option<usize>, T> {
            state,
            options,
            null_label,
            label,
            position,
            required,
            disabled,
            error,
        }
    }
}
