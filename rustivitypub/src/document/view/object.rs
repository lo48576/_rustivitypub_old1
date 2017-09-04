//! Object view.

use document::JsonValue;
use document::view::fetch;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};


/// An object view.
///
/// See [\[REC-activitystreams-core-20170523\] 4.1
/// Object](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#object).
#[derive(Debug, Clone, PartialEq)]
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
