//! Views for value types.

use document::{JsonValue, JsonObject};
use document::view::{Result, PropertyError, TryFromJsonValue};
use document::view::fetch;
use iri::ResolvedIri;


/// An IRI view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IriView<'a> {
    /// Target object.
    object: &'a str,
}

impl<'a> IriView<'a> {
    /// Creates a new `IriView`.
    pub fn new(s: &'a str) -> Self {
        Self { object: s }
    }

    /// Tries to convert to `ResolvedIri`.
    pub fn to_iri(&self) -> Result<ResolvedIri<'a>> {
        Ok(ResolvedIri::try_from_str(self.object)?)
    }
}

impl<'a> TryFromJsonValue<'a> for IriView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::String(ref v) => Ok(Self { object: v }),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::String(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}


/// A view to a natural language value.
///
/// See [\[REC-activitystreams-core-20170523\] 4.7 Natural Language
/// Values](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#naturalLanguageValues).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NaturalLanguageView<'a> {
    /// A simple string without (explicit) language tags.
    String(&'a str),
    /// A map with language tag keys and string values.
    ///
    /// > Every key in the object form *MUST* be a well-formed
    /// > [\[BCP47\]](https://tools.ietf.org/html/bcp47) Language-Tag. The associated values *MUST*
    /// > be strings.
    LangString(LangStringView<'a>),
}

impl<'a> TryFromJsonValue<'a> for NaturalLanguageView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::String(ref s) => Ok(NaturalLanguageView::String(s)),
            JsonValue::Object(ref m) => Ok(NaturalLanguageView::LangString(LangStringView::new(m))),
            _ => Err(PropertyError::TypeMismatch),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::String(_) |
            JsonValue::Object(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}


/// A map with language tag keys and string values.
///
/// > Every key in the object form *MUST* be a well-formed
/// > [\[BCP47\]](https://tools.ietf.org/html/bcp47) Language-Tag. The associated values *MUST*
/// > be strings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LangStringView<'a> {
    /// Target object.
    object: &'a JsonObject,
}

impl<'a> LangStringView<'a> {
    /// Creates a new `IriView`.
    pub fn new(object: &'a JsonObject) -> Self {
        Self { object }
    }

    /// Returns the string associated with the given language tag string.
    pub fn get_raw(&self, raw_tag: &str) -> Result<&'a str> {
        fetch::types::string(self.object.get(raw_tag))
    }

    /// Creates an iterator over the `LangStringView`.
    pub fn iter(&self) -> LangStringViewIter<'a> {
        LangStringViewIter { iter: self.object.iter() }
    }
}

impl<'a> TryFromJsonValue<'a> for LangStringView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::Object(ref v) => Ok(Self { object: v }),
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

impl<'a, 'b> IntoIterator for &'b LangStringView<'a> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = LangStringViewIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


/// Iterator over `LangStringView`.
pub struct LangStringViewIter<'a> {
    /// Iterator.
    iter: <&'a JsonObject as IntoIterator>::IntoIter,
}

impl<'a> Iterator for LangStringViewIter<'a> {
    /// Key is a (raw) language tag, and value is a value string.
    type Item = Result<(&'a str, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((key, &JsonValue::String(ref value))) => Some(Ok((key, value))),
            Some(_) => Some(Err(PropertyError::TypeMismatch)),
            None => None,
        }
    }
}


/// A datetime view.
///
/// See [\[REC-activitystreams-core-20170523\] 2.3 Date and
/// Times](https://www.w3.org/TR/2017/REC-activitystreams-core-20170523/#dates) and [\[RFC3339\]
/// 5.6. Internet Date/Time Format](https://tools.ietf.org/html/rfc3339#section-5.6).
///
/// * as2-partial-time: hh:mm[:ss][.secfrac]
/// * as2-full-time: hh:mm[:ss][.secfrac]["Z" / ("+"/"-")hh:mm]
/// * as2-date-time: YYYY-MM-DDThh:mm[:ss][.secfrac]["Z" / ("+"/"-")hh:mm]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTimeView<'a> {
    /// Target object.
    object: &'a str,
}

impl<'a> DateTimeView<'a> {
    /// Returns the raw string data.
    pub fn raw_str(&self) -> &'a str {
        self.object
    }
}

impl<'a> TryFromJsonValue<'a> for DateTimeView<'a> {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::String(ref s) => Ok(Self { object: s }),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::String(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}
