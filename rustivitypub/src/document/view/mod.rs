//! Object views.

use document::JsonValue;
pub use self::error::{Result, PropertyError};
pub use self::link::LinkView;
pub use self::object::ObjectView;
pub use self::object_or_link::ObjectOrLinkView;
pub use self::value::IriView;

pub mod error;
pub mod link;
pub mod object;
pub mod object_or_link;
pub mod value;


/// Attempt to construct `Self` via a conversion.
///
/// NOTE: Use
/// [`std::convert::TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html)
/// when it is stabilized.
pub trait TryFromJsonValue<'a>: Sized {
    /// Performs the conversion.
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self>;

    /// Checks if the `Self` type object can be created from the given JSON value.
    ///
    /// Returns `Err(_)` if the given JSON value cannot be converted into `Self`.
    /// Note that returned `Ok(())` doesn't always means the type conversion would succeed.
    ///
    /// This should be called at the head of `try_from_json_value()`.
    fn validate_json_value(_value: &JsonValue) -> Result<()> {
        Ok(())
    }
}


/// Fetches an object.
#[inline]
fn fetch_obj(object: Option<&JsonValue>) -> Result<&JsonValue> {
    object.ok_or(PropertyError::NoSuchProperty)
}


/// Fetches an IRI.
#[inline]
fn fetch_iri(object: Option<&JsonValue>) -> Result<IriView> {
    fetch_obj(object).and_then(IriView::try_from_json_value)
}


/// Fetches a string.
#[inline]
fn fetch_str(object: Option<&JsonValue>) -> Result<&str> {
    match *fetch_obj(object)? {
        JsonValue::String(ref s) => Ok(s),
        _ => Err(PropertyError::TypeMismatch),
    }
}
