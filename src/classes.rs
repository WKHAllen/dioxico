use std::fmt::Display;

/// A representation of a list of CSS classes.
pub struct Classes(Vec<String>);

impl Classes {
    /// Creates a new empty list of CSS classes.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Extends the existing list of classes with another.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl From<&str> for Classes {
    fn from(value: &str) -> Self {
        Self(vec![value.to_owned()])
    }
}

impl From<String> for Classes {
    fn from(value: String) -> Self {
        Self(vec![value])
    }
}

impl<T> From<Vec<T>> for Classes
where
    T: Into<Classes>,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().fold(Vec::new(), |mut classes, item| {
            let item_classes: Classes = item.into();
            classes.extend(item_classes.0);
            classes
        }))
    }
}

impl<T> From<Option<T>> for Classes
where
    T: Into<Classes>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(inner) => inner.into(),
            None => Self(vec![]),
        }
    }
}

impl<F, T> From<F> for Classes
where
    F: FnOnce() -> T,
    T: Into<Classes>,
{
    fn from(value: F) -> Self {
        value().into()
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join(" "))
    }
}

/// Creates a list of CSS classes.
macro_rules! classes {
    ( $($class:expr),* ) => {{
        #[allow(unused_mut)]
        let mut classes = $crate::classes::Classes::new();
        $(
            classes.extend($crate::classes::Classes::from($class));
        )*
        classes.to_string()
    }};
}

pub(super) use classes;
