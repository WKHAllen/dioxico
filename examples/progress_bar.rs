use dioxico::ProgressBar;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ProgressBar {
            progress: 0,
        }
        ProgressBar {
            progress: 0.05,
        }
        ProgressBar {
            progress: 0.2,
        }
        ProgressBar {
            progress: 0.5,
        }
        ProgressBar {
            progress: 0.8,
        }
        ProgressBar {
            progress: 0.95,
        }
        ProgressBar {
            progress: 1,
        }
        ProgressBar {
            progress: 0.5,
            disabled: true,
        }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
