//! Fetchers for specific properties.

use document::JsonValue;
use document::consts;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::{NaturalLanguageView, IriView, ObjectOrLinkView, SingleOrMultiView};
use document::view::{DateTimeView, LinkView, MediaTypeView, DurationView, ImageOrLinkView};
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


/// Returns `bcc`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-bcc).
#[inline]
pub fn bcc(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::BCC))
}


/// Returns `bto`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-bto).
#[inline]
pub fn bto(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::BTO))
}


/// Returns `cc`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-cc).
#[inline]
pub fn cc(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::CC))
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


/// Returns `duration`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-duration).
#[inline]
pub fn duration(object: &JsonValue) -> Result<DurationView> {
    types::duration(object.get(consts::DURATION))
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


/// Returns `image`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-image-term).
#[inline]
pub fn image(object: &JsonValue) -> Result<SingleOrMultiView<ImageOrLinkView>> {
    types::single_or_multi_image_or_link(object.get(consts::IMAGE))
}


/// Returns `inReplyTo`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-inreplyto).
#[inline]
pub fn in_reply_to(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::IN_REPLY_TO))
}


/// Returns `location`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-location).
#[inline]
pub fn location(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::LOCATION))
}


/// Returns `name`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-name).
#[inline]
pub fn name(object: &JsonValue) -> Result<NaturalLanguageView> {
    types::natural_language_string(object.get(consts::NAME))
}


/// Returns `mediaType`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-mediatype).
#[inline]
pub fn media_type(object: &JsonValue) -> Result<MediaTypeView> {
    types::media_type(object.get(consts::MEDIA_TYPE))
}


/// Returns `preview`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-preview).
#[inline]
pub fn preview(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::PREVIEW))
}


/// Returns `published`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-published).
#[inline]
pub fn published(object: &JsonValue) -> Result<DateTimeView> {
    types::datetime(object.get(consts::PUBLISHED))
}


/// Returns `replies`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-replies).
#[inline]
pub fn replies(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::REPLIES))
}


/// Returns `start_time`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-starttime).
#[inline]
pub fn start_time(object: &JsonValue) -> Result<DateTimeView> {
    types::datetime(object.get(consts::START_TIME))
}


/// Returns `summary`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-summary).
#[inline]
pub fn summary(object: &JsonValue) -> Result<NaturalLanguageView> {
    types::natural_language_string(object.get(consts::SUMMARY))
}


/// Returns `tag`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-tag).
#[inline]
pub fn tag(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::TAG))
}


/// Returns `to`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-to).
#[inline]
pub fn to(object: &JsonValue) -> Result<SingleOrMultiView<ObjectOrLinkView>> {
    types::single_or_multi_object_or_link(object.get(consts::TO))
}


/// Returns `updated`.
///
/// See [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-updated).
#[inline]
pub fn updated(object: &JsonValue) -> Result<DateTimeView> {
    types::datetime(object.get(consts::UPDATED))
}


/// Returns `url`.
///
/// See [\[REC-activitystreams-core-20170523\] 4.2
/// Link](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#link) and
/// [\[REC-activitystreams-vocabulary-20170523\] 4.
/// Properties](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-url).
#[inline]
pub fn url(object: &JsonValue) -> Result<SingleOrMultiView<LinkView>> {
    types::single_or_multi_link(object.get(consts::URL))
}
