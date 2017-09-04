//! Fetchers for specific properties.

use document::JsonValue;
use document::consts;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::IriView;
use document::view::fetch::types;


/// Returns `id`.
///
/// Returns `Ok(Some(iri))` if `id` is specified as JSON string,
/// returns `Ok(None)` if `id` is explicitly specified as JSON `null`.
///
/// See [\[CR-activitypub-20170822\] 3.1 Object
/// Identifiers](https://www.w3.org/TR/2017/CR-activitypub-20170822/#obj-id).
///
/// > An ID explicitly specified as the JSON `null` object, which implies an anonymous object (a
/// > part of its parent context)
#[inline]
pub fn id(object: &JsonValue) -> Result<Option<IriView>> {
    let id_obj = types::json_obj(object.get(consts::ID))?;
    match *id_obj {
        JsonValue::String(_) => IriView::try_from_json_value(id_obj).map(Some),
        JsonValue::Null => Ok(None),
        _ => Err(PropertyError::TypeMismatch),
    }
}


/// Returns `type` as string.
#[inline]
pub fn type_raw(object: &JsonValue) -> Result<&str> {
    types::string(object.get(consts::TYPE))
}


/// Returns `href`.
///
/// Returns `href` property (string) if the given JSON value is object (map),
/// returns string itself if the given JSON value is string.
///
/// See [\[REC-activitystreams-core-20170523\] 4.2
/// Link](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#link) and
/// [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-href).
#[inline]
pub fn href(object: &JsonValue) -> Result<IriView> {
    match *object {
        JsonValue::Object(ref map) => types::iri(map.get(consts::HREF)),
        JsonValue::String(ref s) => Ok(IriView::new(s)),
        _ => Err(PropertyError::TypeMismatch),
    }
}
