//! Utilities involving generic collection types.

use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashSet, LinkedList, VecDeque};
use std::ops::{Deref, DerefMut};

/// Wrapper type for a variety of collection types, e.g. `Vec<T>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Collection<T> {
    /// The inner collection, stored as a `Vec<T>`.
    inner: Vec<T>,
}

impl<T> Collection<T> {
    /// Consumes the collection and returns the inner `Vec<T>`.
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}

impl<T> Deref for Collection<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Borrow<Vec<T>> for Collection<T> {
    fn borrow(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> BorrowMut<Vec<T>> for Collection<T> {
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T> AsRef<Vec<T>> for Collection<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> AsMut<Vec<T>> for Collection<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T, U> From<&[U]> for Collection<T>
where
    U: Into<T> + Clone,
{
    fn from(value: &[U]) -> Self {
        Self {
            inner: value.iter().map(|x| x.clone().into()).collect(),
        }
    }
}

impl<T, U, const N: usize> From<[U; N]> for Collection<T>
where
    U: Into<T>,
{
    fn from(value: [U; N]) -> Self {
        Self {
            inner: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<T, U> From<Vec<U>> for Collection<T>
where
    U: Into<T>,
{
    fn from(value: Vec<U>) -> Self {
        Self {
            inner: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<T, U> From<VecDeque<U>> for Collection<T>
where
    U: Into<T>,
{
    fn from(value: VecDeque<U>) -> Self {
        Self {
            inner: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<T, U> From<HashSet<U>> for Collection<T>
where
    U: Into<T>,
{
    fn from(value: HashSet<U>) -> Self {
        Self {
            inner: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<T, U> From<LinkedList<U>> for Collection<T>
where
    U: Into<T>,
{
    fn from(value: LinkedList<U>) -> Self {
        Self {
            inner: value.into_iter().map(Into::into).collect(),
        }
    }
}
