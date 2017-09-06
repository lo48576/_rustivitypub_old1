//! `Docuemnt` type object view.

use document::JsonValue;
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::ObjectView;


/// A `Document` type object view.
///
/// See [\[REC-activitystreams-core-20170523\] 4.1
/// Object](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#dfn-document).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DocumentView<'a> {
    /// Target object.
    object: &'a JsonValue,
}

impl<'a> DocumentView<'a> {
    /// Creates a new `DocumentView` from a raw JSON object.
    ///
    /// Use `TryFromJsonValue::try_from_json_value` for normal use.
    pub(crate) fn new(object: &'a JsonValue) -> Self {
        Self { object }
    }

    /// Upcasts the view.
    pub fn object_view(&self) -> ObjectView<'a> {
        ObjectView::new(self.object)
    }
}

impl<'a> TryFromJsonValue<'a> for DocumentView<'a> {
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
