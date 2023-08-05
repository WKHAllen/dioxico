use csscolorparser::Color as CssColor;
use dioxus::prelude::*;
use std::fmt::Display;
use std::ops::Deref;

const ERROR_SIZE_SMALLER_EM: f64 = 0.8;
const ERROR_SIZE_SMALL_EM: f64 = 0.9;
const ERROR_SIZE_MEDIUM_EM: f64 = 1.0;
const ERROR_SIZE_LARGE_EM: f64 = 1.25;
const ERROR_SIZE_LARGER_EM: f64 = 1.5;
const ERROR_COLOR: &str = "#cf0000";
const PADDING_SMALL_PX: f64 = 4.0;
const PADDING_MEDIUM_PX: f64 = 8.0;
const PADDING_LARGE_PX: f64 = 16.0;

#[derive(Debug, Clone, Copy, Default)]
pub enum LengthUnit {
    #[default]
    Px,
    Pt,
    Em,
    Rem,
}

use LengthUnit::*;

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Px => "px",
            Self::Pt => "pt",
            Self::Em => "em",
            Self::Rem => "rem",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Length {
    value: f64,
    units: LengthUnit,
}

impl Length {
    pub fn new(value: f64, units: LengthUnit) -> Self {
        Self { value, units }
    }
}

impl From<(f64, LengthUnit)> for Length {
    fn from((value, units): (f64, LengthUnit)) -> Self {
        Self { value, units }
    }
}

impl From<f64> for Length {
    fn from(value: f64) -> Self {
        Self {
            value,
            units: LengthUnit::default(),
        }
    }
}

impl From<i32> for Length {
    fn from(value: i32) -> Self {
        Self {
            value: value as f64,
            units: LengthUnit::default(),
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.value, self.units))
    }
}

macro_rules! len {
    ( 0 ) => {
        Length::new(0.0, Px)
    };
    ( $val:expr, px ) => {
        Length::new($val, Px)
    };
    ( $val:expr, pt ) => {
        Length::new($val, Pt)
    };
    ( $val:expr, em ) => {
        Length::new($val, Em)
    };
    ( $val:expr, rem ) => {
        Length::new($val, Rem)
    };
}

pub struct Lengths(Vec<Length>);

impl Display for Lengths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

// macro_rules! lens {
//     ( @internal [ $( $elems:expr, )* ] [] ) => {
//         Lengths(vec![ $( $elems, )* ])
//     };
//     ( @internal [ $( $elems:expr, )* ] [$current:expr, $( $rest:expr, )*] ) => {
//         lens!( @internal [ $( $elems, )* Length::from($current), ] [ $( $rest, )* ] )
//     };
//     ( $( $exprs:expr ),* ) => {
//         lens!( @internal [] [$( $exprs, )*] )
//     };
// }
macro_rules! lens {
    ( $( $exprs:expr ),* ) => {
        Lengths(vec![ $( Length::from( $exprs ) ),* ])
    };
}

#[derive(Debug, Clone)]
pub struct Color(CssColor);

impl From<CssColor> for Color {
    fn from(value: CssColor) -> Self {
        Self(value)
    }
}

impl Deref for Color {
    type Target = CssColor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_hex_string())
    }
}

#[derive(Debug, Clone)]
pub struct StyleConstants {
    pub error_size_smaller: Length,
    pub error_size_small: Length,
    pub error_size_medium: Length,
    pub error_size_large: Length,
    pub error_size_larger: Length,
    pub error_color: Color,
    pub padding_small: Length,
    pub padding_medium: Length,
    pub padding_large: Length,
}

impl Default for StyleConstants {
    fn default() -> Self {
        Self {
            error_size_smaller: len!(ERROR_SIZE_SMALLER_EM, em),
            error_size_small: len!(ERROR_SIZE_SMALL_EM, em),
            error_size_medium: len!(ERROR_SIZE_MEDIUM_EM, em),
            error_size_large: len!(ERROR_SIZE_LARGE_EM, em),
            error_size_larger: len!(ERROR_SIZE_LARGER_EM, em),
            error_color: Color::from(ERROR_COLOR.parse::<CssColor>().unwrap()),
            padding_small: len!(PADDING_SMALL_PX, px),
            padding_medium: len!(PADDING_MEDIUM_PX, px),
            padding_large: len!(PADDING_LARGE_PX, px),
        }
    }
}

impl StyleConstants {
    pub fn form_padding(&self) -> Lengths {
        lens!(self.padding_medium, 0)
    }
}

#[derive(Debug, Default)]
pub struct Theme {
    pub styles: StyleConstants,
}

pub fn use_theme(cx: &ScopeState) -> &UseSharedState<Theme> {
    use_shared_state_provider(cx, Theme::default);
    use_shared_state::<Theme>(cx).expect("shared theme state not set")
}
