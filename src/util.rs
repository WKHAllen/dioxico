use js_sys::Math;
use web_sys::{Document, Window};

/// Gets the window object.
pub fn window() -> Window {
    web_sys::window().expect("no window")
}

/// Gets the document object.
pub fn document() -> Document {
    window().document().expect("no document")
}

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = Math::random().to_bits();
    let hex_value = format!("{value:x}");
    hex_value
}

/// Logs to the console.
#[allow(unused_macros)]
macro_rules! console_log {
    ( $($arg:tt)* ) => {{
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($arg)*)));
    }};
}

#[allow(unused_imports)]
pub(crate) use console_log;
