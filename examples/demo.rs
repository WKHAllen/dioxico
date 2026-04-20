use convert_case::{Case, Casing};
use dioxico::ConfigProvider;
use dioxus::prelude::*;

mod button;
mod checkbox;
mod error;
mod icon;
mod icon_button;
mod input;
mod number_input;
mod switch;
mod textarea;
mod theme;

macro_rules! demo_views {
    ( $( $demo:ident ),* $(,)? ) => {
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

#[component]
fn Demo() -> Element {
    let demos = demo_views!(
        theme,
        error,
        input,
        textarea,
        number_input,
        button,
        icon,
        icon_button,
        checkbox,
        switch,
    );

    rsx! {
        Title {
            "Dioxico demo"
        }

        ConfigProvider {
            div {
                class: "dioxico-demo",

                {demos}
            }
        }
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        launch(Demo);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus::desktop::{Config, WindowBuilder};

        let window_config = WindowBuilder::new()
            .with_always_on_bottom(false)
            .with_always_on_top(false)
            .with_title("Dioxico demo");

        let launch_config = Config::new()
            .with_menu(None)
            .with_disable_context_menu(false)
            .with_window(window_config);

        LaunchBuilder::new().with_cfg(launch_config).launch(Demo);
    }
}
