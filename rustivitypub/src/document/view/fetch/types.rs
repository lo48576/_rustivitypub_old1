//! Fetchers for specific types.

use document::JsonValue;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::{NaturalLanguageView, IriView, ObjectOrLinkView, SingleOrMultiView};
use document::view::DateTimeView;


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


/// Returns `SingleOrMultiView<ObjectOrLinkView>`.
#[inline]
pub fn single_or_multi_object_or_link(
    object: Option<&JsonValue>,
) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    SingleOrMultiView::try_from_json_value(json_obj(object)?)
}


/// Returns a natural language view.
#[inline]
pub fn natural_language_string(object: Option<&JsonValue>) -> Result<NaturalLanguageView> {
    NaturalLanguageView::try_from_json_value(json_obj(object)?)
}


/// Returns a `DateTimeView`.
#[inline]
pub fn datetime(object: Option<&JsonValue>) -> Result<DateTimeView> {
    DateTimeView::try_from_json_value(json_obj(object)?)
}
