//! Object view.

use document::JsonValue;
use document::view::fetch;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};
use document::view::{ObjectOrLinkView, SingleOrMultiView, NaturalLanguageView, DateTimeView};


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

    /// Returns `name`.
    ///
    /// See [`document::view::fetch::property::name()`](../fetch/property/fn.name.html).
    pub fn name(&self) -> Result<NaturalLanguageView<'a>> {
        fetch::property::name(self.object)
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
