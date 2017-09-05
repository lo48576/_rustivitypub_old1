//! "Object or link" view.

use document::JsonValue;
use document::view::{Result, TryFromJsonValue, ObjectView, LinkView};


/// A view to an object or a link.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectOrLinkView<'a> {
    /// Target object.
    object: &'a JsonValue,
}

impl<'a> ObjectOrLinkView<'a> {
    /// Creates an object view.
    pub fn object_view(&self) -> Result<ObjectView<'a>> {
        ObjectView::try_from_json_value(self.object)
    }

    /// Creates a link view.
    pub fn link_view(&self) -> Result<LinkView<'a>> {
        LinkView::try_from_json_value(self.object)
    }
}

impl<'a> TryFromJsonValue<'a> for ObjectOrLinkView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        Ok(Self { object: value })
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        ObjectView::try_from_json_value(value)?;
        LinkView::try_from_json_value(value)?;
        Ok(())
    }
}
