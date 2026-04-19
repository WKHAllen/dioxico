use super::{Error, ErrorSize};
use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;
use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;

/// Shortens a number to a specified number of decimal places.
fn shorten_to(value_str: &str, decimals: u16) -> String {
    match value_str.find('.') {
        Some(index) => {
            if index + (decimals as usize) < value_str.len() {
                (value_str[..=index + (decimals as usize)]).to_owned()
            } else {
                value_str.to_owned()
            }
        }
        None => value_str.to_owned(),
    }
}

/// Transforms a string representation of a number as needed.
fn transform_number(value_str: &str, decimals: u16) -> String {
    let mut value_str = shorten_to(value_str, decimals);

    if value_str == "-" {
        value_str = String::new();
    }

    if value_str.ends_with('-') {
        if value_str.starts_with('-') {
            value_str = (value_str[1..value_str.len() - 1]).to_owned();
        } else {
            value_str = format!("-{}", &value_str[..value_str.len() - 1]);
        }
    }

    if value_str.len() > 1 && value_str.starts_with('0') && !value_str.starts_with("0.") {
        value_str = (value_str[1..]).to_owned();
    }

    if value_str.len() > 2 && value_str.starts_with("-0") && !value_str.starts_with("-0.") {
        value_str = format!("-{}", (&value_str[2..]));
    }

    value_str
}

/// Parses the value of a string representation of a number in a text input box.
fn parse_number_value<N>(value_str: &str, min: N, max: N) -> Option<(N, bool)>
where
    N: Number,
{
    match value_str.parse::<N>() {
        Ok(value) => {
            if value < min {
                Some((min, true))
            } else if value > max {
                Some((max, true))
            } else {
                Some((value, false))
            }
        }
        Err(_) => None,
    }
}

/// Parses a string representation of a number in a text input box.
fn parse_number<N>(value_str: &str, min: N, max: N) -> Option<(N, bool)>
where
    N: Number,
{
    if value_str.is_empty() {
        Some((N::default(), true))
    } else if N::DECIMAL && value_str.ends_with('.') && value_str.matches('.').count() == 1 {
        parse_number_value(&value_str[..value_str.len() - 1], min, max)
    } else {
        parse_number_value(value_str, min, max)
    }
}

/// A wrapper around a number state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberState<N>
where
    N: Number,
{
    /// The inner state string.
    state: String,
    /// The inner state value.
    value: N,
    /// The minimum value.
    min: N,
    /// The maximum value.
    max: N,
    /// The maximum number of digits after the decimal.
    decimals: u16,
}

impl<N> NumberState<N>
where
    N: Number,
{
    /// Creates a new number state.
    pub fn new(value: N) -> Self {
        Self::default().value(value)
    }

    /// Sets the state value.
    #[must_use]
    pub fn value(mut self, value: N) -> Self {
        self.state = value.to_string();
        self.value = value;
        self
    }

    /// Sets the minimum value.
    #[must_use]
    pub const fn min(mut self, min: N) -> Self {
        self.min = min;
        self
    }

    /// Sets the maximum value.
    #[must_use]
    pub const fn max(mut self, max: N) -> Self {
        self.max = max;
        self
    }

    /// Sets the maximum number of digits after the decimal.
    #[must_use]
    pub const fn decimals(mut self, decimals: u16) -> Self {
        self.decimals = decimals;
        self
    }

    /// Sets the inner state.
    #[allow(clippy::missing_panics_doc)]
    pub fn set(&mut self, new_value_str: &str) {
        let new_value_transformed = transform_number(new_value_str, self.decimals);
        let maybe_new_value = parse_number(&new_value_transformed, self.min, self.max);

        if let Some((new_value, update_repr)) = maybe_new_value {
            if !update_repr {
                self.state = new_value_transformed;
            } else {
                self.state = new_value.to_string();
            }

            self.value = parse_number(&self.state, self.min, self.max).unwrap().0;
        }
    }

    /// Sets the inner state by value.
    #[inline]
    pub fn set_value(&mut self, new_value: N) {
        self.set(&new_value.to_string());
    }
}

impl<N> Deref for NumberState<N>
where
    N: Number,
{
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<N> Borrow<N> for NumberState<N>
where
    N: Number,
{
    fn borrow(&self) -> &N {
        &self.value
    }
}

impl<N> Display for NumberState<N>
where
    N: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.state.is_empty() {
            f.write_str(&N::default().to_string())
        } else {
            f.write_str(&self.state)
        }
    }
}

impl<N> Default for NumberState<N>
where
    N: Number,
{
    fn default() -> Self {
        Self {
            state: String::new(),
            value: N::default(),
            min: N::NUMBER_MIN,
            max: N::NUMBER_MAX,
            decimals: u16::MAX,
        }
    }
}

impl<N> From<N> for NumberState<N>
where
    N: Number,
{
    fn from(value: N) -> Self {
        Self::default().value(value)
    }
}

/// A number input component.
#[component]
pub fn NumberInput<N>(
    #[props(into)] state: State<NumberState<N>>,
    #[props(default)] label: String,
    #[props(default)] placeholder: String,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] error: String,
) -> Element
where
    N: Number + 'static,
{
    let id = use_id();
    let invalid = !error.is_empty();

    rsx! {
        div {
            class: classes!("dioxico-input-container", disabled.then_some("dioxico-input-container-disabled")),

            label {
                class: "dioxico-input-label",
                r#for: "{id}",

                "{label}"

                span {
                    class: "dioxico-required-mark",

                    if required {
                        " *"
                    }
                }
            }

            div {
                class: "dioxico-input-box-container",

                input {
                    r#type: "text",
                    class: classes!("dioxico-input", invalid.then_some("dioxico-input-invalid")),
                    id,
                    value: "{state.get()}",
                    oninput: move |event| {
                        let new_value_str = event.value();
                        state.with_mut(move |num| num.set(&new_value_str));
                    },
                    placeholder,
                    required,
                    disabled,
                }
            }

            Error {
                message: error,
                size: ErrorSize::Small,
            }
        }
    }
}
