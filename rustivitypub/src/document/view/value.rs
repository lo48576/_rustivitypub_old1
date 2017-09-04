//! Views for value types.

use document::JsonValue;
use document::view::{Result, PropertyError, TryFromJsonValue};
use iri::ResolvedIri;


/// An IRI view.
#[derive(Debug, Clone, PartialEq)]
pub struct IriView<'a> {
    /// Target object.
    object: &'a str,
}

impl<'a> IriView<'a> {
    /// Creates a new `IriView`.
    pub fn new(s: &'a str) -> Self {
        Self { object: s }
    }

    /// Tries to convert to `ResolvedIri`.
    pub fn to_iri(&self) -> Result<ResolvedIri<'a>> {
        Ok(ResolvedIri::try_from_str(self.object)?)
    }
}

impl<'a> TryFromJsonValue<'a> for IriView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::String(ref v) => Ok(Self { object: v }),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::String(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}
