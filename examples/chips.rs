use dioxico::{Chips, ChipsPopupPosition};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state = use_signal(Vec::new);
    let options = vec![
        "C",
        "C++",
        "C#",
        "Java",
        "JavaScript/TypeScript",
        "Python",
        "Go",
        "Rust",
    ];
    let selected_values = state
        .read()
        .iter()
        .map(|index| options[*index])
        .collect::<Vec<_>>()
        .join(", ");
    let error = if state.read().is_empty() {
        "Please select at least one language"
    } else {
        ""
    };

    rsx! {
        Chips {
            state,
            options: options.clone(),
            label: "Chips label",
            option_display_limit: 5,
            max_selections: 6,
            placeholder: "Placeholder!",
            error,
        }
        span {
            "Selected: {selected_values}"
        }
        Chips {
            state,
            options: options.clone(),
            label: "Disabled chips label",
            disabled: true,
        }
        Chips {
            state,
            options: options.clone(),
            label: "Chips above",
            position: ChipsPopupPosition::Above,
            error,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
