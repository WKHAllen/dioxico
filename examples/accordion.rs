use dioxico::Accordion;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let state1 = use_signal(|| false);
    let state2 = use_signal(|| false);
    let state3 = use_signal(|| false);
    let state4 = use_signal(|| true);

    rsx! {
        Accordion {
            state: state1,
            title: "Accordion 1",

            p { "Lorem ipsum etc etc." }
        }
        Accordion {
            state: state2,
            title: "Accordion 2",

            div { "foo" }
            div { "bar" }
            div { "baz" }
        }
        Accordion {
            state: state3,
            title: "Disabled accordion",
            disabled: true,

            span { "This should never be viewable" }
        }
        Accordion {
            state: state4,
            title: "Disabled open accordion",
            disabled: true,

            span { "This should always be viewable" }
        }
        div { "State 1: {state1()}" }
        div { "State 2: {state2()}" }
        div { "State 3: {state3()}" }
        div { "State 4: {state4()}" }
    }
}

#[allow(dead_code)]
fn main() {
    launch(Demo);
}
