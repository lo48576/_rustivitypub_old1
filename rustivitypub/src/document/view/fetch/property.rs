//! Fetchers for specific properties.

use document::JsonValue;
use document::consts;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::{NaturalLanguageView, IriView, ObjectOrLinkView, SingleOrMultiView};
use document::view::DateTimeView;
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


/// Returns `attachment`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-attachment).
#[inline]
pub fn attachment(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::ATTACHMENT))
}


/// Returns `attributedTo`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-attributedto).
#[inline]
pub fn attributed_to(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::ATTRIBUTED_TO))
}


/// Returns `audience`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-audience).
#[inline]
pub fn audience(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::AUDIENCE))
}


/// Returns `content`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-content).
#[inline]
pub fn content(object: &JsonValue) -> Result<NaturalLanguageView> {
    types::natural_language_string(object.get(consts::CONTENT))
}


/// Returns `context`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-context).
#[inline]
pub fn context(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::CONTEXT))
}


/// Returns `endTime`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-endtime).
#[inline]
pub fn end_time(object: &JsonValue) -> Result<DateTimeView> {
    types::datetime(object.get(consts::END_TIME))
}


/// Returns `generator`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-generator).
#[inline]
pub fn generator(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::GENERATOR))
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


/// Returns `icon`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-icon).
#[inline]
pub fn icon(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::ICON))
}


/// Returns `name`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-name).
#[inline]
pub fn name(object: &JsonValue) -> Result<NaturalLanguageView> {
    types::natural_language_string(object.get(consts::NAME))
}
