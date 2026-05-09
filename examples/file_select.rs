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
            text: "Primary file select button",
            on_select: move |files| state.set(files),
        }
        FileSelect {
            text: "Secondary file select button",
            style: FileSelectButtonStyle::Secondary,
            directory: true,
            on_select: move |files| state.set(files),
        }
        FileSelect {
            text: "Transparent file select button",
            style: FileSelectButtonStyle::Transparent,
            multiple: true,
            on_select: move |files| state.set(files),
        }
        FileSelect {
            text: "Danger file select button",
            style: FileSelectButtonStyle::Danger,
            accept: [".rs".to_owned()],
            on_select: move |files| state.set(files),
        }
        FileSelect {
            text: "Disabled file select button",
            disabled: true,
            on_select: move |files| state.set(files),
        }
        FileDrop {
            text: "File drop zone",
            on_drop: move |files| state.set(files),
        }
        FileDrop {
            text: "Disabled file drop zone",
            disabled: true,
            on_drop: move |files| state.set(files),
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
