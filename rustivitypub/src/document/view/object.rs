//! Object view.

use document::JsonValue;
use document::view::fetch;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};
use document::view::{ObjectOrLinkView, SingleOrMultiView, NaturalLanguageView, DateTimeView};
use document::view::{LinkView, MediaTypeView, ImageOrLinkView};


/// An object view.
///
/// See [\[REC-activitystreams-core-20170523\] 4.1
/// Object](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#object).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectView<'a> {
    /// Target object.
    object: &'a JsonValue,
}

impl<'a> ObjectView<'a> {
    /// Creates a new `ObjectView` from a raw JSON object.
    ///
    /// Use `TryFromJsonValue::try_from_json_value` for normal use.
    pub(crate) fn new(object: &'a JsonValue) -> Self {
        Self { object }
    }

    /// Returns `id`.
    ///
    /// See [`document::view::fetch::property::id()`](../fetch/property/fn.id.html).
    pub fn id(&self) -> Result<Option<IriView<'a>>> {
        fetch::property::id(self.object)
    }

    /// Returns `type` as string.
    ///
    /// See [`document::view::fetch::property::type_raw()`](../fetch/property/fn.type_raw.html).
    pub fn type_raw(&self) -> Result<&'a str> {
        fetch::property::type_raw(self.object)
    }

    /// Returns `attachment`.
    ///
    /// See [`document::view::fetch::property::attachment()`](../fetch/property/fn.attachment.html).
    pub fn attachment(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::attachment(self.object)
    }

    /// Returns `audience`.
    ///
    /// See [`document::view::fetch::property::audience()`](../fetch/property/fn.audience.html).
    pub fn audience(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::audience(self.object)
    }

    /// Returns `attributedTo`.
    ///
    /// See [`document::view::fetch::property::attributed_to()`](../fetch/property/fn.attributed_to.html).
    pub fn attributed_to(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::attributed_to(self.object)
    }

    /// Returns `bto`.
    ///
    /// See [`document::view::fetch::property::to()`](../fetch/property/fn.bto.html).
    pub fn bto(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::bto(self.object)
    }

    /// Returns `content`.
    ///
    /// See [`document::view::fetch::property::content()`](../fetch/property/fn.content.html).
    pub fn content(&self) -> Result<NaturalLanguageView<'a>> {
        fetch::property::content(self.object)
    }

    /// Returns `context`.
    ///
    /// See [`document::view::fetch::property::context()`](../fetch/property/fn.context.html).
    pub fn context(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::context(self.object)
    }

    /// Returns `endTime`.
    ///
    /// See [`document::view::fetch::property::end_time()`](../fetch/property/fn.end_time.html).
    pub fn end_time(&self) -> Result<DateTimeView<'a>> {
        fetch::property::end_time(self.object)
    }

    /// Returns `generator`.
    ///
    /// See [`document::view::fetch::property::generator()`](../fetch/property/fn.generator.html).
    pub fn generator(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::generator(self.object)
    }

    /// Returns `icon`.
    ///
    /// See [`document::view::fetch::property::icon()`](../fetch/property/fn.icon.html).
    pub fn icon(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::icon(self.object)
    }

    /// Returns `image`.
    ///
    /// See [`document::view::fetch::property::image()`](../fetch/property/fn.image.html).
    pub fn image(&self) -> Result<SingleOrMultiView<'a, ImageOrLinkView>> {
        fetch::property::image(self.object)
    }

    /// Returns `inReplyTo`.
    ///
    /// See [`document::view::fetch::property::in_reply_to()`](../fetch/property/fn.in_reply_to.html).
    pub fn in_reply_to(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::in_reply_to(self.object)
    }

    /// Returns `location`.
    ///
    /// See [`document::view::fetch::property::location()`](../fetch/property/fn.location.html).
    pub fn location(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::location(self.object)
    }

    /// Returns `name`.
    ///
    /// See [`document::view::fetch::property::name()`](../fetch/property/fn.name.html).
    pub fn name(&self) -> Result<NaturalLanguageView<'a>> {
        fetch::property::name(self.object)
    }

    /// Returns `mediaType`.
    ///
    /// > When used on an Object, identifies the MIME media type of the value of the content
    /// > property. If not specified, the content property is assumed to contain text/html content.
    ///
    /// See [`document::view::fetch::property::media_type()`](../fetch/property/fn.media_type.html).
    pub fn media_type(&self) -> Result<MediaTypeView<'a>> {
        fetch::property::media_type(self.object)
    }

    /// Returns `preview`.
    ///
    /// See [`document::view::fetch::property::preview()`](../fetch/property/fn.preview.html).
    pub fn preview(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::preview(self.object)
    }

    /// Returns `published`.
    ///
    /// See [`document::view::fetch::property::published()`](../fetch/property/fn.published.html).
    pub fn published(&self) -> Result<DateTimeView<'a>> {
        fetch::property::published(self.object)
    }

    /// Returns `replies`.
    ///
    /// See [`document::view::fetch::property::replies()`](../fetch/property/fn.replies.html).
    pub fn replies(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::replies(self.object)
    }

    /// Returns `startTime`.
    ///
    /// See [`document::view::fetch::property::start_time()`](../fetch/property/fn.starttime.html).
    pub fn start_time(&self) -> Result<DateTimeView<'a>> {
        fetch::property::start_time(self.object)
    }

    /// Returns `summary`.
    ///
    /// See [`document::view::fetch::property::summary()`](../fetch/property/fn.summary.html).
    pub fn summary(&self) -> Result<NaturalLanguageView<'a>> {
        fetch::property::summary(self.object)
    }

    /// Returns `tag`.
    ///
    /// See [`document::view::fetch::property::tag()`](../fetch/property/fn.tag.html).
    pub fn tag(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::tag(self.object)
    }

    /// Returns `to`.
    ///
    /// See [`document::view::fetch::property::to()`](../fetch/property/fn.to.html).
    pub fn to(&self) -> Result<SingleOrMultiView<'a, ObjectOrLinkView>> {
        fetch::property::to(self.object)
    }

    /// Returns `updated`.
    ///
    /// See [`document::view::fetch::property::updated()`](../fetch/property/fn.updated.html).
    pub fn updated(&self) -> Result<DateTimeView<'a>> {
        fetch::property::updated(self.object)
    }

    /// Returns `url`.
    ///
    /// See [`document::view::fetch::property::url()`](../fetch/property/fn.url.html).
    pub fn url(&self) -> Result<SingleOrMultiView<'a, LinkView>> {
        fetch::property::url(self.object)
    }
}

impl<'a> TryFromJsonValue<'a> for ObjectView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::Object(_) => Ok(Self { object: value }),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::Object(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}
