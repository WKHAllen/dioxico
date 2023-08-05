use convert_case::{Case, Casing};
use dioxus::prelude::*;

mod error;
mod input;
mod theme;

macro_rules! demo_views {
    ( $( $demo:ident ),* ) => {
        rsx! {
            $({
                use crate::$demo::Demo;
                let name = stringify!($demo);
                let name_title = name.to_case(Case::Title);
                rsx! {
                    div {
                        class: "dioxico-demo-item",

                        span {
                            class: "dioxico-demo-item-label",

                            "{name_title}"
                        }

                        Demo {}
                    }
                }
            })*
        }
    };
}

#[allow(non_snake_case)]
fn Demo(cx: Scope) -> Element {
    let demos = demo_views!(theme, error, input);

    cx.render(rsx! {
        div {
            class: "dioxico-demo",

            demos
        }
    })
}

fn main() {
    dioxus_web::launch(Demo);
}
