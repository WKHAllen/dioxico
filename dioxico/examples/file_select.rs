use dioxico::{FileData, FileDrop, FileSelect, FileSelectButtonStyle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(Vec::<FileData>::new);
    let paths = state
        .read()
        .iter()
        .map(|file| file.path().display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    rsx! {
        FileSelect {
            on_select: move |files| state.set(files),

            "Primary file select button"
        }
        FileSelect {
            style: FileSelectButtonStyle::Secondary,
            directory: true,
            on_select: move |files| state.set(files),

            "Secondary file select button"
        }
        FileSelect {
            style: FileSelectButtonStyle::Transparent,
            multiple: true,
            on_select: move |files| state.set(files),

            "Transparent file select button"
        }
        FileSelect {
            style: FileSelectButtonStyle::Danger,
            accept: [".rs".to_owned()],
            on_select: move |files| state.set(files),

            "Danger file select button"
        }
        FileSelect {
            disabled: true,
            on_select: move |files| state.set(files),

            "Disabled file select button"
        }
        FileDrop {
            on_drop: move |files| state.set(files),

            "File drop zone"
        }
        FileDrop {
            disabled: true,
            on_drop: move |files| state.set(files),

            "Disabled file drop zone"
        }
        span {
            "Selected files: {paths}"
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
