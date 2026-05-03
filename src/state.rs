use dioxus::core::{MarkerWrapper, SpawnIfAsync, SuperFrom};
use dioxus::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

pub enum StateRef<'a, T, S = UnsyncStorage>
where
    S: Storage<SignalData<T>>,
{
    ValueRef(&'a T),
    SignalRef(ReadableRef<'a, Signal<T, S>, T>),
}

impl<'a, T, S> Clone for StateRef<'a, T, S>
where
    S: Storage<SignalData<T>>,
    S::Ref<'a, T>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::ValueRef(value) => Self::ValueRef(*value),
            Self::SignalRef(sigref) => Self::SignalRef(sigref.clone()),
        }
    }
}

impl<'a, T, S> Copy for StateRef<'a, T, S>
where
    S: Storage<SignalData<T>>,
    S::Ref<'a, T>: Copy,
{
}

impl<'a, T, S> Deref for StateRef<'a, T, S>
where
    S: Storage<SignalData<T>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::ValueRef(value) => value,
            Self::SignalRef(sigref) => sigref,
        }
    }
}

impl<'a, T, S> Borrow<T> for StateRef<'a, T, S>
where
    S: Storage<SignalData<T>>,
{
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T, S> AsRef<T> for StateRef<'a, T, S>
where
    S: Storage<SignalData<T>>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

pub enum StateMut<'a, T, S = UnsyncStorage>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    ValueMut(&'a mut T, EventHandler<T>),
    SignalMut(WritableRef<'a, Signal<T, S>, T>),
}

impl<'a, T, S> Deref for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::ValueMut(value, _) => value,
            Self::SignalMut(sigmut) => sigmut,
        }
    }
}

impl<'a, T, S> DerefMut for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::ValueMut(value, _) => value,
            Self::SignalMut(sigmut) => sigmut,
        }
    }
}

impl<'a, T, S> Borrow<T> for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T, S> BorrowMut<T> for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<'a, T, S> AsRef<T> for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T, S> AsMut<T> for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<'a, T, S> Drop for StateMut<'a, T, S>
where
    T: Clone + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    fn drop(&mut self) {
        if let Self::ValueMut(value, callback) = self {
            callback.call(value.clone());
        }
    }
}

/// A generalized stateful value, primarily intended to be used in controlled
/// inputs.
#[derive(Debug)]
pub enum State<T, S = UnsyncStorage>
where
    T: Clone + PartialEq + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    /// An initial value.
    Value(T, T),
    /// An intial value and change callback.
    ValueCallback(T, T, EventHandler<T>),
    /// A signal wrapping a value.
    Signal(Signal<T, S>),
}

impl<T, S> Clone for State<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Value(value, value2) => Self::Value(value.clone(), value2.clone()),
            Self::ValueCallback(value, value2, callback) => {
                Self::ValueCallback(value.clone(), value2.clone(), *callback)
            }
            Self::Signal(sig) => Self::Signal(*sig),
        }
    }
}

impl<T, S> Copy for State<T, S>
where
    T: Clone + Copy + PartialEq + 'static,
    S: Storage<SignalData<T>> + 'static,
{
}

impl<T, S> PartialEq for State<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Storage<SignalData<T>> + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.read().eq(&other.read())
    }
}

impl<T> State<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Returns a clone of the underlying value.
    pub fn get(&self) -> T {
        match self {
            Self::Value(value, _) => value.clone(),
            Self::ValueCallback(value, _, _) => value.clone(),
            Self::Signal(sig) => sig(),
        }
    }

    /// Sets the underlying value.
    pub fn set(&mut self, new_value: T) {
        match self {
            Self::Value(_, _) => {}
            Self::ValueCallback(_, _, callback) => callback.call(new_value),
            Self::Signal(sig) => sig.set(new_value),
        }
    }

    /// Runs a closure that immutably borrows the inner value.
    pub fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        match self {
            Self::Value(value, _) => f(value),
            Self::ValueCallback(value, _, _) => f(value),
            Self::Signal(sig) => f(&sig.read()),
        }
    }

    /// Runs a closure that mutably borrows the inner value.
    pub fn with_mut<O>(&mut self, f: impl FnOnce(&mut T) -> O) -> O {
        match self {
            Self::Value(value, value2) => {
                *value2 = value.clone();
                f(value2)
            }
            Self::ValueCallback(value, value2, callback) => {
                *value2 = value.clone();
                let ret = f(value2);

                if *value != *value2 {
                    callback.call(value2.clone());
                }

                ret
            }
            Self::Signal(sig) => f(&mut sig.write()),
        }
    }
}

impl<T, S> State<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Storage<SignalData<T>>,
{
    /// Gets an immutable reference to the stateful value.
    pub fn read(&self) -> StateRef<'_, T, S> {
        match self {
            Self::Value(value, _) => StateRef::ValueRef(value),
            Self::ValueCallback(_, value2, _) => StateRef::ValueRef(value2),
            Self::Signal(sig) => StateRef::SignalRef(sig.read()),
        }
    }
}

impl<T, S> State<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Storage<SignalData<T>>,
{
    /// Gets a mutable reference to the stateful value.
    pub fn write(&mut self) -> StateMut<'_, T, S> {
        match self {
            Self::Value(value, value2) => {
                *value2 = value.clone();
                StateMut::ValueMut(value2, EventHandler::default())
            }
            Self::ValueCallback(value, value2, callback) => {
                *value2 = value.clone();
                StateMut::ValueMut(value2, *callback)
            }
            Self::Signal(sig) => StateMut::SignalMut(sig.write()),
        }
    }
}

impl<T> From<T> for State<T>
where
    T: Clone + PartialEq + 'static,
{
    fn from(value: T) -> Self {
        Self::Value(value.clone(), value)
    }
}

impl From<&str> for State<String> {
    fn from(value: &str) -> Self {
        Self::Value(value.to_owned(), value.to_owned())
    }
}

impl From<&String> for State<String> {
    fn from(value: &String) -> Self {
        Self::Value(value.clone(), value.clone())
    }
}

impl<T, F, Spawn, Marker> SuperFrom<(T, F), MarkerWrapper<Marker>> for State<T>
where
    T: Clone + PartialEq + 'static,
    F: FnMut(T) -> Spawn + 'static,
    Spawn: SpawnIfAsync<Marker> + 'static,
{
    fn super_from(input: (T, F)) -> Self {
        Self::ValueCallback(input.0.clone(), input.0, Callback::new(input.1))
    }
}

impl<F, Spawn, Marker> SuperFrom<(&str, F), MarkerWrapper<Marker>> for State<String>
where
    F: FnMut(String) -> Spawn + 'static,
    Spawn: SpawnIfAsync<Marker> + 'static,
{
    fn super_from(input: (&str, F)) -> Self {
        Self::ValueCallback(
            input.0.to_owned(),
            input.0.to_owned(),
            Callback::new(input.1),
        )
    }
}

impl<F, Spawn, Marker> SuperFrom<(&String, F), MarkerWrapper<Marker>> for State<String>
where
    F: FnMut(String) -> Spawn + 'static,
    Spawn: SpawnIfAsync<Marker> + 'static,
{
    fn super_from(input: (&String, F)) -> Self {
        Self::ValueCallback(input.0.clone(), input.0.clone(), Callback::new(input.1))
    }
}

impl<T> From<Signal<T>> for State<T>
where
    T: Clone + PartialEq + 'static,
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
        Self::Value(value.clone())
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
    T: Clone + PartialEq + 'static,
{
    fn from(value: State<T>) -> Self {
        match value {
            State::Value(value, _) => Self::Value(value),
            State::ValueCallback(value, _, _) => Self::Value(value),
            State::Signal(sig) => Self::Signal(sig.into()),
        }
    }
}
