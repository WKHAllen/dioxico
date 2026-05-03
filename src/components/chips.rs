use super::{Error, ErrorSize, IconButton, IconButtonSize, XMARK_ICON};
use crate::classes::*;
use crate::hooks::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;
use std::fmt::Display;

/// Compares an option to a typed out value, returning a score indicating the
/// strength of the match, or `None` if the strings do not match.
fn option_match(option: &str, value: &str) -> Option<usize> {
    let option = option.to_lowercase();
    let value = value.to_lowercase();
    let mut score = 0;
    let mut indices_since_last_match = 0;
    let option_chars = option.chars();
    let mut value_chars = value.chars().peekable();
    let mut any_match = false;

    if option == value {
        return Some(0);
    }

    for option_char in option_chars {
        indices_since_last_match += 1;

        match value_chars.peek() {
            Some(value_char) => {
                if option_char == *value_char {
                    score += indices_since_last_match;
                    indices_since_last_match = 0;
                    value_chars.next();
                    any_match = true;
                }
            }
            None => break,
        }
    }

    if any_match && value_chars.next().is_none() {
        Some(score)
    } else {
        None
    }
}

/// Limits the number of options.
fn limit_options<T>(options: &[T], display_limit: Option<usize>) -> &[T] {
    let limit_index = if let Some(display_limit) = display_limit {
        if options.len() > display_limit {
            display_limit
        } else {
            options.len()
        }
    } else {
        options.len()
    };

    &options[..limit_index]
}

/// Returns a list of possible options, taking into account the complete list
/// of options, the currently selected options, and the option the user has
/// begun to type out.
fn get_possible_options(
    all_options: &[impl Display],
    selected_options_indices: &[usize],
    next_option: &str,
    display_limit: Option<usize>,
    max_selections: Option<usize>,
) -> Vec<usize> {
    if let Some(max_selections) = max_selections {
        if selected_options_indices.len() >= max_selections {
            return Vec::new();
        }
    }

    let unselected_options_indices = (0..all_options.len())
        .filter(|index| !selected_options_indices.contains(index))
        .collect::<Vec<_>>();

    if next_option.is_empty() {
        return limit_options(&unselected_options_indices, display_limit).to_owned();
    }

    let mut matches = unselected_options_indices
        .into_iter()
        .filter_map(|index| {
            all_options.get(index).and_then(|option| {
                option_match(&option.to_string(), next_option).map(|score| (index, score))
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));

    let limited_matches = limit_options(&matches, display_limit);

    limited_matches.iter().map(|(option, _)| *option).collect()
}

/// Position of a chips popup.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChipsPopupPosition {
    /// Position the popup above.
    Above,
    /// Position the popup below.
    #[default]
    Below,
}

impl ChipsPopupPosition {
    /// Gets the name of the position.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Above => "above",
            Self::Below => "below",
        }
    }
}

/// Chips selection component.
#[component]
pub fn Chips<T>(
    /// Chips selection state.
    #[props(into)]
    state: State<Vec<usize>>,
    /// List of chips options.
    #[props(into)]
    options: Vec<T>,
    /// Maximum number of options ot display in the dropdown.
    option_display_limit: Option<usize>,
    /// Maximum number of options that can be selected.
    max_selections: Option<usize>,
    /// Chips selection label.
    #[props(default)]
    label: String,
    /// Placeholder text for when the selection is empty.
    #[props(default)]
    placeholder: String,
    /// Popup position.
    #[props(default)]
    position: ChipsPopupPosition,
    /// Maximum number of characters allowed in the chips input.
    #[props(default = 524288)]
    max_length: usize,
    /// Is this chips selection input disabled?
    #[props(default)]
    disabled: bool,
    /// An optional error message. If missing or empty, no error will be shown.
    #[props(default)]
    error: String,
) -> Element
where
    T: Display + Clone + PartialEq + 'static,
{
    let id = use_id();
    let mut dropdown_open = use_signal(|| false);
    let mut next_chip = use_signal(String::new);

    let possible_options = get_possible_options(
        &options,
        &state.read(),
        &next_chip.read(),
        option_display_limit,
        max_selections,
    );

    let invalid = !error.is_empty();

    // TODO: enable selecting via arrow keys, space/enter, and escape/backspace
    // let mut selecting = use_signal(|| None);

    let chips_node_onclick = use_click_away(move || dropdown_open.set(false));

    rsx! {
        div {
            class: classes!("dioxico-chips-container", disabled.then_some("dioxico-chips-container-disabled"), dropdown_open().then_some("dioxico-chips-container-open"), invalid.then_some("dioxico-chips-container-invalid")),

            div {
                class: "dioxico-chips-label-container",

                label {
                    r#for: "{id}",
                    class: "dioxico-chips-label",

                    {label}
                }
            }

            div {
                class: classes!("dioxico-chips", format!("dioxico-chips-{}", position.as_str())),
                onclick: chips_node_onclick,
                // TODO: node ref?

                div {
                    class: "dioxico-chips-inner",

                    if !state.read().is_empty() {
                        div {
                            class: "dioxico-chips-chip-list",

                            for (index, this_chip_index) in state.read().iter().enumerate() {
                                div {
                                    class: "dioxico-chips-chip",

                                    span {
                                        class: "dioxico-chips-chip-label",

                                        {options.get(*this_chip_index).map(ToString::to_string).unwrap_or_default()}
                                    }

                                    IconButton {
                                        icon: XMARK_ICON,
                                        size: IconButtonSize::Small,
                                        class: "dioxico-chips-chip-remove",
                                        on_click: {
                                            let mut state = state.clone();
                                            move |_| {
                                                state.write().remove(index);
                                            }
                                        },
                                        disabled,
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "dioxico-chips-input-box-container",

                        input {
                            r#type: "text",
                            class: "dioxico-chips-input",
                            id,
                            value: next_chip,
                            oninput: move |event| next_chip.set(event.value()),
                            onfocusin: move |_| dropdown_open.set(true),
                            maxlength: max_length,
                            placeholder,
                            disabled,
                        }
                    }
                }

                if !possible_options.is_empty() {
                    div {
                        class: "dioxico-chips-options-dropdown",

                        div {
                            class: "dioxico-chips-options-popup",

                            for this_option_index in possible_options {
                                div {
                                    class: "dioxico-chips-option",
                                    onclick: {
                                        let mut state = state.clone();
                                        move |_| {
                                            state.write().push(this_option_index);
                                            next_chip.set(String::new());
                                        }
                                    },

                                    {options.get(this_option_index).map(|x| x.to_string()).unwrap_or_default()}
                                }
                            }
                        }
                    }
                }
            }

            Error {
                message: error,
                size: ErrorSize::Small,
                class: "dioxico-chips-error",
            }
        }
    }
}
