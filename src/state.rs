use dioxus::core::{MarkerWrapper, SpawnIfAsync, SuperFrom};
use dioxus::prelude::*;

/// A generalized stateful value, primarily intended to be used in controlled
/// inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State<T>
where
    T: Clone + 'static,
{
    /// An initial value.
    Value(T),
    /// An intial value and change callback.
    ValueCallback(T, EventHandler<T>),
    /// A signal wrapping a value.
    Signal(Signal<T>),
}

impl<T> State<T>
where
    T: Clone + 'static,
{
    /// Returns a clone of the underlying value.
    pub fn get(&self) -> T {
        match self {
            Self::Value(value) => value.clone(),
            Self::ValueCallback(value, _) => value.clone(),
            Self::Signal(sig) => sig(),
        }
    }

    /// Sets the underlying value.
    pub fn set(&mut self, new_value: T) {
        match self {
            Self::Value(_) => {}
            Self::ValueCallback(_, callback) => callback.call(new_value),
            Self::Signal(sig) => sig.set(new_value),
        }
    }
}

impl<T> From<T> for State<T>
where
    T: Clone + 'static,
{
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for State<String> {
    fn from(value: &str) -> Self {
        Self::Value(value.to_owned())
    }
}

impl From<&String> for State<String> {
    fn from(value: &String) -> Self {
        Self::Value(value.to_owned())
    }
}

impl<T, F, Spawn, Marker> SuperFrom<(T, F), MarkerWrapper<Marker>> for State<T>
where
    T: Clone + 'static,
    F: FnMut(T) -> Spawn + 'static,
    Spawn: SpawnIfAsync<Marker> + 'static,
{
    fn super_from(input: (T, F)) -> Self {
        Self::ValueCallback(input.0, Callback::new(input.1))
    }
}

impl<T> From<Signal<T>> for State<T>
where
    T: Clone + 'static,
{
    fn from(value: Signal<T>) -> Self {
        Self::Signal(value)
    }
}

/// A generalized read-only stateful value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadState<T>
where
    T: Clone + 'static,
{
    /// An initial value.
    Value(T),
    /// A signal wrapping a value.
    Signal(ReadSignal<T>),
}

impl<T> ReadState<T>
where
    T: Clone + 'static,
{
    /// Returns a clone of the underlying value.
    pub fn get(&self) -> T {
        match self {
            Self::Value(value) => value.clone(),
            Self::Signal(sig) => sig(),
        }
    }
}

impl<T> From<T> for ReadState<T>
where
    T: Clone + 'static,
{
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for ReadState<String> {
    fn from(value: &str) -> Self {
        Self::Value(value.to_owned())
    }
}

impl From<&String> for ReadState<String> {
    fn from(value: &String) -> Self {
        Self::Value(value.to_owned())
    }
}

impl<T> From<ReadSignal<T>> for ReadState<T>
where
    T: Clone + 'static,
{
    fn from(value: ReadSignal<T>) -> Self {
        Self::Signal(value)
    }
}

impl<T> From<Signal<T>> for ReadState<T>
where
    T: Clone + 'static,
{
    fn from(value: Signal<T>) -> Self {
        Self::Signal(value.into())
    }
}

impl<T> From<State<T>> for ReadState<T>
where
    T: Clone + 'static,
{
    fn from(value: State<T>) -> Self {
        match value {
            State::Value(value) => Self::Value(value),
            State::ValueCallback(value, _) => Self::Value(value),
            State::Signal(sig) => Self::Signal(sig.into()),
        }
    }
}
