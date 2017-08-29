//! IRI types.
// This implementation is based on `str`, `String`, `std::str` and `std::path`
// on rust-1.19.0 .
// See:
//
//  * https://github.com/rust-lang/rust/blob/1.19.0/src/libcore/str/mod.rs
//  * https://github.com/rust-lang/rust/blob/1.19.0/src/libcollections/string.rs
//  * https://github.com/rust-lang/rust/blob/1.19.0/src/libstd/path.rs

use std::borrow::{Borrow, ToOwned, Cow};
use std::cmp;
use std::error;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use url;


/// An IRI slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iri {
    inner: str,
}

impl Iri {
    /// Converts a string slice to an IRI slice without checking that the string contains valid
    /// IRI.
    #[inline]
    pub unsafe fn from_str_unchecked(s: &str) -> &Self {
        ::std::mem::transmute(s)
    }

    /// Converts a string slice to an IRI slice.
    ///
    /// This method internally creates `url::Url` then discard it.
    /// If you may want `url::Url` later, use `ResolvedIri::try_from_str` instead.
    #[inline]
    pub fn try_from_str(s: &str) -> Result<&Self, ParseError> {
        ResolvedIri::try_from_str(s).map(|v| v.iri)
    }

    /// Converts an `Iri` to an owned `IriBuf`.
    #[inline]
    pub fn to_iri_buf(&self) -> IriBuf {
        IriBuf { inner: self.inner.to_owned() }
    }

    /// Converts an `Iri` to an `url::Url`.
    #[inline]
    pub fn to_url(&self) -> url::Url {
        url::Url::parse(self).expect(
            "Failed to convert an `Iri` to an `Url::url` (should never happen)",
        )
    }
}

impl AsRef<str> for Iri {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<Iri> for Iri {
    fn as_ref(&self) -> &Iri {
        self
    }
}

impl Deref for Iri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ToOwned for Iri {
    type Owned = IriBuf;

    fn to_owned(&self) -> Self::Owned {
        self.to_iri_buf()
    }
}

impl<'a> From<&'a Iri> for url::Url {
    fn from(v: &Iri) -> Self {
        v.to_url()
    }
}


/// An owned IRI.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IriBuf {
    inner: String,
}

impl IriBuf {
    /// Converts a string to an owned IRI without checking that the string contains valid IRI.
    #[inline]
    pub unsafe fn from_string_unchecked(s: String) -> Self {
        IriBuf { inner: s }
    }

    /// Converts a string to an owned IRI.
    ///
    /// This method internally creates `url::Url` then discard it.
    /// If you may want `url::Url` later, use `ResolvedIri::try_from_str` instead.
    // TODO: Use `TryFrom` if stabilized. (See https://github.com/rust-lang/rust/issues/33417 .)
    #[inline]
    pub fn try_from_string(s: String) -> Result<Self, ParseError> {
        ResolvedIriBuf::try_from_string(s).map(|v| v.iri)
    }

    /// Coerces to an `Iri` slice.
    #[inline]
    pub fn as_iri(&self) -> &Iri {
        unsafe { Iri::from_str_unchecked(self.inner.as_str()) }
    }

    /// Converts an `IriBuf` to an `url::Url`.
    #[inline]
    pub fn to_url(&self) -> url::Url {
        url::Url::parse(self).expect(
            "Failed to convert an `IriBuf` to an `Url::url` (should never happen)",
        )
    }
}

impl AsRef<str> for IriBuf {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<Iri> for IriBuf {
    fn as_ref(&self) -> &Iri {
        self
    }
}

impl Borrow<Iri> for IriBuf {
    fn borrow(&self) -> &Iri {
        self.as_iri()
    }
}

impl Deref for IriBuf {
    type Target = Iri;

    fn deref(&self) -> &Self::Target {
        self.as_iri()
    }
}

impl<'a> From<&'a Iri> for IriBuf {
    fn from(v: &'a Iri) -> Self {
        v.to_owned()
    }
}

impl FromStr for IriBuf {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Iri::try_from_str(s).map(ToOwned::to_owned)
    }
}

impl From<IriBuf> for url::Url {
    fn from(v: IriBuf) -> Self {
        v.to_url()
    }
}


// Based on https://github.com/rust-lang/rust/blob/1.19.0/src/libstd/path.rs#L2424-L2460
macro_rules! impl_cmp {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { <Iri as PartialEq>::eq(self, other) }
        }
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { <Iri as PartialEq>::eq(self, other) }
        }
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <Iri as PartialOrd>::partial_cmp(self, other)
            }
        }
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <Iri as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(IriBuf, Iri);
impl_cmp!(IriBuf, &'a Iri);
impl_cmp!(Cow<'a, Iri>, Iri);
impl_cmp!(Cow<'a, Iri>, &'b Iri);
impl_cmp!(Cow<'a, Iri>, IriBuf);


/// A raw IRI slice and an owned resolved IRI (which is directly usable as URL).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolvedIri<'a> {
    /// IRI.
    iri: &'a Iri,
    /// Resolved IRI.
    url: url::Url,
}

impl<'a> ResolvedIri<'a> {
    /// Tries to convert a string slice to a `ResolvedIri`.
    // TODO: Use `TryFrom` if stabilized. (See https://github.com/rust-lang/rust/issues/33417 .)
    #[inline]
    pub fn try_from_str(s: &'a str) -> Result<Self, ParseError> {
        let url = run_iri_validation(s)?;
        let iri = unsafe { Iri::from_str_unchecked(s) };
        Ok(ResolvedIri { iri, url })
    }

    /// Deconstructs the `ResolvedIri` into inner `Iri` and `Url`.
    #[inline]
    pub fn into_inner(self) -> (&'a Iri, url::Url) {
        (self.iri, self.url)
    }

    /// Returns a reference to the inner IRI.
    #[inline]
    pub fn as_iri(&self) -> &'a Iri {
        self.iri
    }

    /// Returns a reference to the inner `Url`, which contains resolved IRI.
    #[inline]
    pub fn as_url(&self) -> &url::Url {
        &self.url
    }
}

impl<'a> AsRef<Iri> for ResolvedIri<'a> {
    fn as_ref(&self) -> &Iri {
        self.as_iri()
    }
}

impl<'a> AsRef<url::Url> for ResolvedIri<'a> {
    fn as_ref(&self) -> &url::Url {
        self.as_url()
    }
}


/// An owned raw IRI and an owned resolved IRI (which is directly usable as URL).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolvedIriBuf {
    /// Owned IRI.
    iri: IriBuf,
    /// Resolved IRI.
    url: url::Url,
}

impl ResolvedIriBuf {
    /// Tries to convert a string slice to a `ResolvedIriBuf`.
    // TODO: Use `TryFrom` if stabilized. (See https://github.com/rust-lang/rust/issues/33417 .)
    #[inline]
    pub fn try_from_string(s: String) -> Result<Self, ParseError> {
        let url = run_iri_validation(&s)?;
        let iri = IriBuf { inner: s };
        Ok(ResolvedIriBuf { iri, url })
    }

    /// Deconstructs the `ResolvedIri` into inner `Iri` and `Url`.
    #[inline]
    pub fn into_inner(self) -> (IriBuf, url::Url) {
        (self.iri, self.url)
    }

    /// Returns a reference to the inner IRI.
    #[inline]
    pub fn as_iri(&self) -> &Iri {
        &self.iri
    }

    /// Returns a reference to the inner `Url`, which contains resolved IRI.
    #[inline]
    pub fn as_url(&self) -> &url::Url {
        &self.url
    }
}

impl AsRef<Iri> for ResolvedIriBuf {
    fn as_ref(&self) -> &Iri {
        self.as_iri()
    }
}

impl AsRef<url::Url> for ResolvedIriBuf {
    fn as_ref(&self) -> &url::Url {
        self.as_url()
    }
}

impl<'a> From<ResolvedIri<'a>> for ResolvedIriBuf {
    fn from(v: ResolvedIri<'a>) -> Self {
        Self {
            iri: v.iri.into(),
            url: v.url,
        }
    }
}

impl FromStr for ResolvedIriBuf {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ResolvedIri::try_from_str(s).map(Into::into)
    }
}


/// An IRI parse error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Parse error from the `url` crate.
    Url(url::ParseError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ParseError::Url(ref e) = *self;
        write!(f, "Provided string was invalid as IRI: {}", e)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        let ParseError::Url(ref e) = *self;
        error::Error::description(e)
    }
}

impl From<url::ParseError> for ParseError {
    fn from(v: url::ParseError) -> Self {
        ParseError::Url(v)
    }
}


/// Checks whether the given string is valid IRI.
fn run_iri_validation(s: &str) -> Result<url::Url, ParseError> {
    Ok(url::Url::parse(s)?)
}
