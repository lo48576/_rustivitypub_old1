//! Single or multiple values view.

use std::marker::PhantomData;

use document::JsonValue;
use document::view::{TryFromJsonValue, Result};


/// A view to a single value or multi values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SingleOrMultiJsonView<'a> {
    /// Single value.
    Single(&'a JsonValue),
    /// Multi values.
    Multi(&'a [JsonValue]),
}

impl<'a> SingleOrMultiJsonView<'a> {
    /// Creates a view from a reference to an object.
    pub fn new(object: &'a JsonValue) -> Self {
        match *object {
            JsonValue::Array(ref arr) => SingleOrMultiJsonView::Multi(arr),
            ref obj => SingleOrMultiJsonView::Single(obj),
        }
    }

    /// Creates an iterator over the values.
    pub fn iter(&self) -> SingleOrMultiJsonViewIter<'a> {
        match *self {
            SingleOrMultiJsonView::Single(obj) => SingleOrMultiJsonViewIter::Single(
                ::std::iter::once(obj),
            ),
            SingleOrMultiJsonView::Multi(arr) => SingleOrMultiJsonViewIter::Multi(arr.iter()),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b SingleOrMultiJsonView<'a> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = SingleOrMultiJsonViewIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


/// `SingleOrMultiJsonView` iterator.
#[derive(Debug, Clone)]
pub enum SingleOrMultiJsonViewIter<'a> {
    /// Single value iterator.
    Single(::std::iter::Once<&'a JsonValue>),
    /// Multi values iterator.
    Multi(::std::slice::Iter<'a, JsonValue>),
}

impl<'a> Iterator for SingleOrMultiJsonViewIter<'a> {
    type Item = &'a JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            SingleOrMultiJsonViewIter::Single(ref mut iter) => iter.next(),
            SingleOrMultiJsonViewIter::Multi(ref mut iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match *self {
            SingleOrMultiJsonViewIter::Single(ref iter) => iter.size_hint(),
            SingleOrMultiJsonViewIter::Multi(ref iter) => iter.size_hint(),
        }
    }

    fn count(self) -> usize {
        match self {
            SingleOrMultiJsonViewIter::Single(iter) => iter.count(),
            SingleOrMultiJsonViewIter::Multi(iter) => iter.count(),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self {
            SingleOrMultiJsonViewIter::Single(iter) => iter.last(),
            SingleOrMultiJsonViewIter::Multi(iter) => iter.last(),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match *self {
            SingleOrMultiJsonViewIter::Single(ref mut iter) => iter.nth(n),
            SingleOrMultiJsonViewIter::Multi(ref mut iter) => iter.nth(n),
        }
    }
}

impl<'a> ExactSizeIterator for SingleOrMultiJsonViewIter<'a> {}

impl<'a> DoubleEndedIterator for SingleOrMultiJsonViewIter<'a> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        match *self {
            SingleOrMultiJsonViewIter::Single(ref mut iter) => iter.next_back(),
            SingleOrMultiJsonViewIter::Multi(ref mut iter) => iter.next_back(),
        }
    }
}


/// A view to a single object or multiple objects.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SingleOrMultiView<'a, T> {
    /// Target object.
    object: SingleOrMultiJsonView<'a>,
    /// Target view type.
    _phantom: PhantomData<T>,
}

impl<'a, T> SingleOrMultiView<'a, T>
where
    T: TryFromJsonValue<'a>,
{
    /// Creates an iterator over the values.
    pub fn iter(&self) -> SingleOrMultiViewIter<'a, T> {
        SingleOrMultiViewIter {
            iter: self.object.iter(),
            _t: PhantomData,
        }
    }
}

impl<'a, T> TryFromJsonValue<'a> for SingleOrMultiView<'a, T>
where
    T: TryFromJsonValue<'a>,
{
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        Ok(Self {
            object: SingleOrMultiJsonView::new(value),
            _phantom: PhantomData,
        })
    }
}

impl<'a, 'b, T> IntoIterator for &'b SingleOrMultiView<'a, T>
where
    T: TryFromJsonValue<'a>,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = SingleOrMultiViewIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


/// `SingleOrMultiView` iterator.
#[derive(Debug, Clone)]
pub struct SingleOrMultiViewIter<'a, T> {
    /// Iterator.
    iter: SingleOrMultiJsonViewIter<'a>,
    /// A view type to be returned.
    _t: PhantomData<T>,
}

impl<'a, T> Iterator for SingleOrMultiViewIter<'a, T>
where
    T: TryFromJsonValue<'a>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(T::try_from_json_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(T::try_from_json_value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(T::try_from_json_value)
    }
}

impl<'a, T> ExactSizeIterator for SingleOrMultiViewIter<'a, T>
where
    T: TryFromJsonValue<'a>,
{
}

impl<'a, T> DoubleEndedIterator for SingleOrMultiViewIter<'a, T>
where
    T: TryFromJsonValue<'a>,
{
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.iter.next().map(T::try_from_json_value)
    }
}
