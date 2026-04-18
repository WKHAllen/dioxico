use dioxus::prelude::*;

/// Generates a random ID for an element.
fn new_id() -> String {
    let value = rand::random::<u32>();
    let hex_value = format!("{value:x}");
    hex_value
}

pub fn use_id() -> String {
    use_hook(new_id)
}
