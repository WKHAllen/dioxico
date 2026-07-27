use dioxico::{
    Select, SelectEnum, SelectEnumNullable, SelectNullable, SelectSearchable,
    SelectSearchableNullable, UnitEnum,
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, UnitEnum)]
    enum Language {
        C,
        Zig,
        Go,
        Rust,
    }

    let select_state = use_signal(|| 0usize);
    let select_error = if select_state() == 2 {
        "Please select something other than 3"
    } else {
        ""
    };
    let select_nullable_state = use_signal(|| None::<usize>);
    let options = (1..=5)
        .map(|index| format!("Option {}", index))
        .collect::<Vec<_>>();
    let select_enum_state = use_signal(|| Language::C);
    let select_enum_nullable_state = use_signal(|| None::<Language>);
    let mut select_searchable_state = use_signal(|| 0usize);
    let mut select_searchable_nullable_state = use_signal(|| None);
    let select_searchable_error = if select_searchable_nullable_state().is_none() {
        "Please select an option"
    } else {
        ""
    };

    rsx! {
      Select {
        state: select_state,
        options: options.clone(),
        label: "Select label",
        required: true,
        error: select_error,
      }
      span { "Value: {select_state():?}" }
      SelectNullable {
        state: select_nullable_state,
        options: options.clone(),
        null_label: "Select an option...",
        label: "Select nullable label",
      }
      span { "Value: {select_nullable_state():?}" }
      SelectNullable {
        state: select_nullable_state,
        options: options.clone(),
        label: "Disabled select label",
        disabled: true,
      }
      SelectEnum::<Language> {
        state: select_enum_state,
        label: "Select enum label",
        required: true,
      }
      span { "Value: {select_enum_state():?}" }
      SelectEnumNullable {
        state: select_enum_nullable_state,
        null_label: "Select an option...",
        label: "Select enum nullable label",
      }
      span { "Value: {select_enum_nullable_state():?}" }
      SelectEnumNullable {
        state: select_enum_nullable_state,
        label: "Disabled select enum label",
        disabled: true,
      }
      SelectSearchable {
        state: select_searchable_state,
        options: options.clone(),
        on_submit: move |results: Vec<usize>| {
            if let Some(index) = results.first() {
                select_searchable_state.set(*index);
            }
        },
        label: "Select searchable label",
        required: true,
      }
      span { "Value: {select_searchable_state():?}" }
      SelectSearchableNullable {
        state: select_searchable_nullable_state,
        options: options.clone(),
        on_submit: move |results: Vec<usize>| {
            if let Some(index) = results.first() {
                select_searchable_nullable_state.set(Some(*index));
            } else {
                select_searchable_nullable_state.set(None);
            }
        },
        null_label: "Select an option...",
        label: "Select searchable nullable label",
        error: select_searchable_error,
      }
      span { "Value: {select_searchable_nullable_state():?}" }
      SelectSearchableNullable {
        state: select_searchable_nullable_state,
        options,
        label: "Disabled select searchable label",
        disabled: true,
      }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
