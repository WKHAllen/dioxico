//! Select dropdown components and utilities.

use super::{Error, ErrorSize, Icon, ANGLE_DOWN_ICON};
use crate::classes::*;
use crate::collection::Collection;
use crate::css_repr::CssRepr;
use crate::element::ElementLike;
use crate::hooks::*;
use crate::state::State;
use crate::unit_enum::UnitEnum;
use crate::util::*;
use dioxico_macros::CssRepr;
use dioxus::core::SuperFrom;
use dioxus::prelude::*;
use std::cmp::Ordering;
use std::rc::Rc;

/// Helper trait to note which types can be used to track the state of a select
/// component.
trait SelectState: Copy {
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CssRepr)]
pub enum SelectPopupPosition {
    /// Position the popup above.
    Above,
    /// Position the popup below.
    #[default]
    Below,
}

/// Select dropdown component.
#[component]
fn SelectInner<S>(
    /// Selection state.
    #[props(into)]
    state: State<S>,
    /// List of select options.
    options: Collection<ElementLike>,
    /// Null label option element. Defaults to "Select...".
    #[props(default = ElementLike::super_from("Select..."))]
    null_label: ElementLike,
    /// Select label element.
    #[props(default)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element
where
    S: SelectState + Clone + PartialEq + 'static,
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
        Some(selected) => options.get(selected).cloned().unwrap_or_else({
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
        class: classes!(
            "dioxico-select-container", disabled
            .then_some("dioxico-select-container-disabled"), dropdown_open()
            .then_some("dioxico-select-container-open"), class
        ),

        div { class: "dioxico-select-label-container",

          label { r#for: "{id}", class: "dioxico-select-label",

            {label}

            span { class: "dioxico-required-mark",

              if required {
                " *"
              }
            }
          }
        }

        div {
          class: classes!("dioxico-select", format!("dioxico-select-{}", position.css_repr())),
          onclick: select_node_onclick,

          button {
            r#type: "button",
            class: classes!(
                "dioxico-select-button", invalid.then_some("dioxico-select-button-invalid")
            ),
            id,
            disabled,
            onclick: move |_| {
                if !disabled {
                    dropdown_open.set(!dropdown_open());
                }
            },

            div { class: "dioxico-select-button-selection", {selected_option} }

            Icon {
              icon: ANGLE_DOWN_ICON,
              disabled,
              class: "dioxico-select-button-icon",
            }
          }

          div { class: "dioxico-select-dropdown",

            div { class: "dioxico-select-popup",
              // onmounted: move |element| {
              //     popup_node.set(Some(element.data()));
              // },
              if S::HAS_NULL_OPTION {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      state.write().set_null_value();
                      dropdown_open.set(false);
                  },

                  {null_label}
                }
              }

              for (index , option) in options.into_inner().into_iter().enumerate() {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      state.write().set_value(index);
                      dropdown_open.set(false);
                  },

                  {option}
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
pub fn Select(
    /// Selection state.
    #[props(into)]
    state: State<usize>,
    /// List of select options.
    #[props(into)]
    options: Collection<ElementLike>,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
      SelectInner::<usize> {
        state,
        options,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}

/// Select dropdown component.
#[component]
pub fn SelectNullable(
    /// Selection state.
    #[props(into)]
    state: State<Option<usize>>,
    /// List of select options.
    #[props(into)]
    options: Collection<ElementLike>,
    /// Null label option element. Defaults to "Select...".
    #[props(default = ElementLike::super_from("Select..."), into)]
    null_label: ElementLike,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
      SelectInner::<Option<usize>> {
        state,
        options,
        null_label,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}

/// Select dropdown component.
#[component]
fn SelectEnumInner<E>(
    /// Selection value.
    value: Option<E>,
    /// Selection change handler.
    #[props(into)]
    on_change: EventHandler<Option<E>>,
    /// Is this enum state nullable?
    nullable: bool,
    /// Null label option element. Defaults to "Select...".
    #[props(default = ElementLike::super_from("Select..."))]
    null_label: ElementLike,
    /// Select label element.
    #[props(default)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element
where
    E: UnitEnum + Clone + PartialEq + 'static,
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

    let selected_option = match value {
        Some(selected) => ElementLike::from(selected.variant_name().to_owned()),
        None => null_label.clone(),
    };

    let select_node_onclick = use_click_away(move || dropdown_open.set(false));

    // TODO: `use_popup`?
    // let mut popup_node = use_signal(|| None);

    rsx! {
      div {
        class: classes!(
            "dioxico-select-container", disabled
            .then_some("dioxico-select-container-disabled"), dropdown_open()
            .then_some("dioxico-select-container-open"), class
        ),

        div { class: "dioxico-select-label-container",

          label { r#for: "{id}", class: "dioxico-select-label",

            {label}

            span { class: "dioxico-required-mark",

              if required {
                " *"
              }
            }
          }
        }

        div {
          class: classes!("dioxico-select", format!("dioxico-select-{}", position.css_repr())),
          onclick: select_node_onclick,

          button {
            r#type: "button",
            class: classes!(
                "dioxico-select-button", invalid.then_some("dioxico-select-button-invalid")
            ),
            id,
            disabled,
            onclick: move |_| {
                if !disabled {
                    dropdown_open.set(!dropdown_open());
                }
            },

            div { class: "dioxico-select-button-selection", {selected_option} }

            Icon {
              icon: ANGLE_DOWN_ICON,
              disabled,
              class: "dioxico-select-button-icon",
            }
          }

          div { class: "dioxico-select-dropdown",

            div { class: "dioxico-select-popup",
              // onmounted: move |element| {
              //     popup_node.set(Some(element.data()));
              // },
              if nullable {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      on_change.call(None);
                      dropdown_open.set(false);
                  },

                  {null_label}
                }
              }

              for option in E::VARIANT_NAMES {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      on_change.call(E::from_variant_name(option));
                      dropdown_open.set(false);
                  },

                  {*option}
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
pub fn SelectEnum<E>(
    /// Selection state.
    #[props(into)]
    state: State<E>,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element
where
    E: UnitEnum + Clone + PartialEq + 'static,
{
    rsx! {
      SelectEnumInner::<E> {
        value: Some(state.get()),
        on_change: move |new_value: Option<E>| state.set(new_value.unwrap()),
        nullable: false,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}

/// Select dropdown component.
#[component]
pub fn SelectEnumNullable<E>(
    /// Selection state.
    #[props(into)]
    state: State<Option<E>>,
    /// Null label option element. Defaults to "Select...".
    #[props(default = ElementLike::super_from("Select..."), into)]
    null_label: ElementLike,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element
where
    E: UnitEnum + Clone + PartialEq + 'static,
{
    rsx! {
      SelectEnumInner::<E> {
        value: state.get(),
        on_change: move |new_value: Option<E>| state.set(new_value),
        nullable: true,
        null_label,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}

/// A result in an option search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SearchResult<'a, T> {
    /// A reference to the matching value.
    pub value: &'a T,
    /// The length of the option.
    pub key_len: usize,
    /// The index in the collection in which this value resides.
    pub collection_index: usize,
    /// The starting index in the value where the query matched.
    pub query_index: usize,
}

/// The default search algorithm. This first prioritizes how early in the result
/// the search query was found, then how short the result was, and finally falls
/// back to the original order. In a case where the collection comes from a
/// database query, it may be useful to have the query return results
/// alphabetically for the sake of this fallback case.
fn search<'a, T, F>(query: &str, collection: &'a [T], mut key_fn: F) -> Vec<SearchResult<'a, T>>
where
    F: FnMut(&T) -> &str,
{
    let query_lower = query.to_lowercase();

    let mut search_values = collection
        .iter()
        .enumerate()
        .filter_map(|(collection_index, value)| {
            let key = key_fn(value).to_lowercase();
            key.find(&query_lower).map(|query_index| SearchResult {
                value,
                key_len: key.len(),
                collection_index,
                query_index,
            })
        })
        .collect::<Vec<_>>();

    search_values.sort_by(|a, b| {
        // Highest sort priority is how early in the result the search query was found
        match a.query_index.cmp(&b.query_index) {
            // Next priority is how short the result is
            Ordering::Equal => match a.key_len.cmp(&b.key_len) {
                // Finally, fallback to original order
                Ordering::Equal => a.collection_index.cmp(&b.collection_index),
                other => other,
            },
            other => other,
        }
    });

    search_values
}

/// A function used to search values in a select dropdown.
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct SearchFn(Rc<dyn Fn(&str, &[String]) -> Vec<usize>>);

impl PartialEq for SearchFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<F> From<F> for SearchFn
where
    F: Fn(&str, &[String]) -> Vec<usize> + 'static,
{
    fn from(value: F) -> Self {
        Self(Rc::new(value))
    }
}

impl Default for SearchFn {
    fn default() -> Self {
        Self(Rc::new(move |query, collection| {
            search(query, collection, String::as_str)
                .into_iter()
                .map(|result| result.collection_index)
                .collect()
        }))
    }
}

/// Select dropdown component.
#[component]
fn SelectSearchableInner<S>(
    /// Selection state.
    #[props(into)]
    state: State<S>,
    /// List of select options.
    options: ReadSignal<Collection<String>>,
    /// An optional function used to match search results. If not provided, a
    /// sensible default search function is used.
    #[props(default)]
    search_fn: SearchFn,
    /// An optional callback called when the enter key is pressed. The value
    /// passed to the callback is a vector containing the indices of all options
    /// that match the current search.
    #[props(default)]
    on_submit: EventHandler<Vec<usize>>,
    /// Null label option element. Defaults to "Select...".
    #[props(default = "Select...".to_owned())]
    null_label: String,
    /// Select label element.
    #[props(default)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element
where
    S: SelectState + Clone + PartialEq + 'static,
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

    let mut search_query = use_memo(move || {
        // This closure runs everytime the state is changed. We always want to
        // close the popup when this happens.
        dropdown_open.set(false);

        match state.get().get_value() {
            Some(selected) => match options.read().get(selected) {
                Some(option) => option.clone(),
                None => String::new(),
            },
            None => String::new(),
        }
    });
    let query_matches = use_memo(move || (*search_fn.0)(&search_query.read(), &options.read()));

    let select_node_onclick = use_click_away(move || {
        // Reset the query to the current state value when focus leaves
        search_query.set(match state.get().get_value() {
            Some(selected) => match options.read().get(selected) {
                Some(option) => option.clone(),
                None => String::new(),
            },
            None => String::new(),
        });
        dropdown_open.set(false);
    });

    // TODO: `use_popup`?
    // let mut popup_node = use_signal(|| None);

    rsx! {
      div {
        class: classes!(
            "dioxico-select-container", disabled
            .then_some("dioxico-select-container-disabled"), dropdown_open()
            .then_some("dioxico-select-container-open"), class
        ),

        div { class: "dioxico-select-label-container",

          label { r#for: "{id}", class: "dioxico-select-label",

            {label}

            span { class: "dioxico-required-mark",

              if required {
                " *"
              }
            }
          }
        }

        div {
          class: classes!("dioxico-select", format!("dioxico-select-{}", position.css_repr())),
          onclick: select_node_onclick,

          div { class: "dioxico-select-search-container",

            div {
              class: classes!(
                  "dioxico-select-search", invalid.then_some("dioxico-select-search-invalid")
              ),

              input {
                r#type: "text",
                class: "dioxico-select-search-input",
                id: "{id}",
                disabled,
                placeholder: "{null_label}",
                value: search_query(),
                onfocusin: move |_| {
                    if !disabled {
                        dropdown_open.set(true);
                    }
                },
                onfocusout: move |_| {
                    dropdown_open.set(false);
                },
                oninput: move |event| search_query.set(event.value()),
                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        on_submit.call(query_matches());
                    }
                },
              }

              label { r#for: id,
                Icon {
                  icon: ANGLE_DOWN_ICON,
                  disabled,
                  class: "dioxico-select-button-icon",
                }
              }
            }
          }

          div { class: "dioxico-select-dropdown",

            div { class: "dioxico-select-popup",
              // onmounted: move |element| {
              //     popup_node.set(Some(element.data()));
              // },
              if S::HAS_NULL_OPTION {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      state.write().set_null_value();
                      dropdown_open.set(false);
                  },

                  "{null_label}"
                }
              }

              for index in query_matches() {
                div {
                  class: "dioxico-select-option",
                  onclick: move |_| {
                      state.write().set_value(index);
                      dropdown_open.set(false);
                  },

                  "{options.read()[index]}"
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

#[component]
pub fn SelectSearchable(
    /// Selection state.
    #[props(into)]
    state: State<usize>,
    /// List of select options.
    #[props(into)]
    options: Collection<String>,
    /// An optional function used to match search results. If not provided, a
    /// sensible default search function is used.
    #[props(default)]
    search_fn: SearchFn,
    /// An optional callback called when the enter key is pressed. The value
    /// passed to the callback is a vector containing the indices of all options
    /// that match the current search.
    #[props(default)]
    on_submit: EventHandler<Vec<usize>>,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
      SelectSearchableInner::<usize> {
        state,
        options,
        search_fn,
        on_submit,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}

#[component]
pub fn SelectSearchableNullable(
    /// Selection state.
    #[props(into)]
    state: State<Option<usize>>,
    /// List of select options.
    #[props(into)]
    options: Collection<String>,
    /// An optional function used to match search results. If not provided, a
    /// sensible default search function is used.
    #[props(default)]
    search_fn: SearchFn,
    /// An optional callback called when the enter key is pressed. The value
    /// passed to the callback is a vector containing the indices of all options
    /// that match the current search.
    #[props(default)]
    on_submit: EventHandler<Vec<usize>>,
    /// Null label option element. Defaults to "Select...".
    #[props(default = "Select...".to_owned())]
    null_label: String,
    /// Select label element.
    #[props(default, into)]
    label: ElementLike,
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
    /// CSS classes to apply to the base element.
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
      SelectSearchableInner::<Option<usize>> {
        state,
        options,
        search_fn,
        on_submit,
        null_label,
        label,
        position,
        required,
        disabled,
        error,
        class,
      }
    }
}
