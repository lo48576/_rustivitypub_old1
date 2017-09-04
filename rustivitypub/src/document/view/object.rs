//! Object view.

use document::JsonValue;
use document::consts;
use document::view::{Result, PropertyError, TryFromJsonValue, IriView};
use document::view::{fetch_iri, fetch_str};


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
    pub fn id(&self) -> Result<IriView<'a>> {
        fetch_iri(self.object.get(consts::ID))
    }

    /// Returns `type` as string.
    pub fn type_raw(&self) -> Result<&'a str> {
        fetch_str(self.object.get(consts::TYPE))
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
