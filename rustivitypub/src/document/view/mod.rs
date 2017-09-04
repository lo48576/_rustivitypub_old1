//! Object views.

use document::JsonValue;
pub use self::error::{Result, PropertyError};

pub mod error;


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
