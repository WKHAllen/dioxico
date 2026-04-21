use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;
use std::fmt::Display;

/// The orientation of a radio group.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RadioGroupOrientation {
    /// Horizontally oriented.
    Horizontal,
    /// Vertically oriented.
    #[default]
    Vertical,
}

impl RadioGroupOrientation {
    /// Gets the name of the orientation.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Radio group component.
#[component]
pub fn RadioGroup<T>(
    /// Radio group state.
    #[props(into)]
    state: State<Option<usize>>,
    /// List of options in the radio group.
    #[props(into)]
    options: Vec<T>,
    /// Radio group orientation.
    #[props(default)]
    orientation: RadioGroupOrientation,
    /// Is this radio group required?
    #[props(default)]
    required: bool,
    /// Is this radio group disabled?
    #[props(default)]
    disabled: bool,
) -> Element
where
    T: Display + Clone + PartialEq + 'static,
{
    let name = use_id();
    // IDs need to be regenerated with every re-render because the list of
    // options may have changed.
    let index_id_options = options
        .into_iter()
        .enumerate()
        .map(|(index, option)| (index, new_id(), option.to_string()))
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: classes!("dioxico-radio-group", format!("dioxico-radio-group-{}", orientation.as_str())),

            for (index, id, option) in index_id_options {
                div {
                    class: classes!("dioxico-radio-option", disabled.then_some("dioxico-radio-option-disabled")),

                    input {
                        r#type: "radio",
                        class: "dioxico-radio-input",
                        id: "{id}",
                        name: "{name}",
                        value: "{index}",
                        oninput: move |_| state.set(Some(index)),
                        checked: state.get() == Some(index),
                        required,
                        disabled,
                    }

                    label {
                        r#for: id,
                        class: "dioxico-radio-label",

                        {option}
                    }
                }
            }
        }
    }
}
