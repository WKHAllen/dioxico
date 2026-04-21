use super::ProgressBar;
use crate::classes::*;
use crate::state::State;
use crate::util::*;
use dioxus::prelude::*;

/// Slider component.
#[component]
pub fn Slider<N>(
    /// Slider state.
    #[props(into)]
    state: State<N>,
    /// Slider label.
    #[props(default)]
    label: String,
    /// Minimum allowed value.
    #[props(default = N::NUMBER_MIN)]
    min: N,
    /// Maximum allowed value.
    #[props(default = N::NUMBER_MAX)]
    max: N,
    /// Step size.
    #[props(default = N::NUMBER_STEP)]
    step: N,
    /// Is this slider disabled?
    #[props(default)]
    disabled: bool,
) -> Element
where
    N: Number + 'static,
{
    let id = use_id();
    let value = state.get();
    let progress = (value.as_f64() - min.as_f64()) / (max.as_f64() - min.as_f64());
    let width_percentage =
        ((value.as_f64() - min.as_f64()) * 100.0f64) / (max.as_f64() - min.as_f64());
    let thumb_transform_style = format!("left: {width_percentage}%");

    rsx! {
        div {
            class: classes!("dioxico-slider-container", disabled.then_some("dioxico-slider-disabled")),

            label {
                r#for: "{id}",
                class: "dioxico-slider-label",

                {label}
            }

            div {
                class: "dioxico-slider",

                div {
                    class: "dioxico-slider-track",

                    ProgressBar {
                        progress,
                        disabled,
                    }
                }

                div {
                    class: "dioxico-slider-thumb-container",

                    div {
                        class: "dioxico-slider-thumb",
                        style: thumb_transform_style,
                    }

                    input {
                        r#type: "range",
                        class: "dioxico-slider-input",
                        id,
                        value: "{value}",
                        min: "{min}",
                        max: "{max}",
                        step: "{step}",
                        oninput: move |event: Event<FormData>| {
                            let new_value = event
                                .value()
                                .parse::<N>()
                                .map_err(|_| format!("failed to parse '{}' as a number", event.value()))
                                .unwrap();
                            state.set(new_value);
                        },
                        disabled,
                    }
                }
            }
        }
    }
}
