//! Link view.

use document::JsonValue;
use document::view::fetch;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};
use document::view::{SingleOrMultiView, NaturalLanguageView, LanguageTagView, MediaTypeView};
use document::view::ObjectOrLinkView;


/// A link view.
///
/// Note that the link can be a plain URI string or a JSON object.
///
/// See [\[REC-activitystreams-core-20170523\] 4.2
/// Link](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#link) and
/// [\[REC-activitystreams-vocabulary-20170523\] 2. Core
/// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-link).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinkView<'a> {
    /// Target object.
    object: &'a JsonValue,
}

impl<'a> LinkView<'a> {
    /// Returns `type` as string.
    ///
    /// See [`document::view::fetch::property::type_raw()`](../fetch/property/fn.type_raw.html).
    pub fn type_raw(&self) -> Result<&'a str> {
        fetch::property::type_raw(self.object)
    }

    /// Returns an IRI to the link target.
    ///
    /// See [`document::view::fetch::property::href()`](../fetch/property/fn.href.html).
    pub fn href(&self) -> Result<IriView<'a>> {
        fetch::property::href(self.object)
    }

    /// Returns `height`.
    ///
    /// See [`document::view::fetch::property::height()`](../fetch/property/fn.height.html).
    pub fn height(&self) -> Result<u64> {
        fetch::property::height(self.object)
    }

    /// Returns `hreflang`.
    ///
    /// See [`document::view::fetch::property::hreflang()`](../fetch/property/fn.hreflang.html).
    pub fn hreflang(&self) -> Result<LanguageTagView<'a>> {
        fetch::property::hreflang(self.object)
    }

    /// Returns `mediaType`.
    ///
    /// > When used on a Link, identifies the MIME media type of the referenced resource.
    ///
    /// See [`document::view::fetch::property::media_type()`](../fetch/property/fn.media_type.html).
    pub fn media_type(&self) -> Result<MediaTypeView<'a>> {
        fetch::property::media_type(self.object)
    }

    /// Returns `name`.
    ///
    /// See [`document::view::fetch::property::name()`](../fetch/property/fn.name.html).
    pub fn name(&self) -> Result<NaturalLanguageView<'a>> {
        fetch::property::name(self.object)
    }

    /// Returns `preview`.
    ///
    /// See [`document::view::fetch::property::preview()`](../fetch/property/fn.preview.html).
    pub fn preview(&self) -> Result<SingleOrMultiView<ObjectOrLinkView<'a>>> {
        fetch::property::preview(self.object)
    }

    /// Returns `rel`.
    ///
    /// See [`document::view::fetch::property::rel()`](../fetch/property/fn.rel.html).
    pub fn rel(&self) -> Result<SingleOrMultiView<&'a str>> {
        fetch::property::rel(self.object)
    }

    /// Returns `width`.
    ///
    /// See [`document::view::fetch::property::width()`](../fetch/property/fn.width.html).
    pub fn width(&self) -> Result<u64> {
        fetch::property::width(self.object)
    }
}

impl<'a> TryFromJsonValue<'a> for LinkView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::Object(_) |
            JsonValue::String(_) => Ok(Self { object: value }),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::Object(_) |
            JsonValue::String(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}
