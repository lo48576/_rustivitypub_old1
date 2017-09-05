//! Fetchers for specific types.

use document::JsonValue;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::{NaturalLanguageView, IriView};


/// Returns a JSON object (map).
#[inline]
pub fn json_obj(object: Option<&JsonValue>) -> Result<&JsonValue> {
    object.ok_or(PropertyError::NoSuchProperty)
}


/// Returns an IRI.
#[inline]
pub fn iri(object: Option<&JsonValue>) -> Result<IriView> {
    json_obj(object).and_then(IriView::try_from_json_value)
}


/// Returns a string.
#[inline]
pub fn string(object: Option<&JsonValue>) -> Result<&str> {
    match *json_obj(object)? {
        JsonValue::String(ref s) => Ok(s),
        _ => Err(PropertyError::TypeMismatch),
    }
}


/// Returns a natural language view.
#[inline]
pub fn natural_language_string(object: Option<&JsonValue>) -> Result<NaturalLanguageView> {
    NaturalLanguageView::try_from_json_value(json_obj(object)?)
}
