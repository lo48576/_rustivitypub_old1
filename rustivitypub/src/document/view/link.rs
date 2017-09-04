//! Link view.

use document::JsonValue;
use document::view::fetch;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};


/// A link view.
///
/// Note that the link can be a plain URI string or a JSON object.
///
/// See [\[REC-activitystreams-core-20170523\] 4.2
/// Link](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#link) and
/// [\[REC-activitystreams-vocabulary-20170523\] 2. Core
/// Types](https://www.w3.org/TR/2017/REC-activitystreams-vocabulary-20170523/#dfn-link).
#[derive(Debug, Clone, PartialEq)]
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
