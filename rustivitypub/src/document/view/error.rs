//! View error and result.

use std::error;
use std::fmt;

use iri;


/// A property fetch result.
pub type Result<T> = ::std::result::Result<T, PropertyError>;


/// A property fetch error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyError {
    /// IRI parse error.
    Iri(iri::ParseError),
    /// Property not found.
    NoSuchProperty,
    /// Unexpected type.
    TypeMismatch,
    /// Unknown document type.
    UnknownDocumentType,
}

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PropertyError::Iri(ref e) => write!(f, "IRI parse error: {}", e),
            PropertyError::NoSuchProperty => write!(f, "Property not found"),
            PropertyError::TypeMismatch => write!(f, "Unexpected type of property"),
            PropertyError::UnknownDocumentType => write!(f, "Unknown document type"),
        }
    }
}

impl error::Error for PropertyError {
    fn description(&self) -> &str {
        match *self {
            PropertyError::Iri(ref e) => error::Error::description(e),
            PropertyError::NoSuchProperty => "Property not found",
            PropertyError::TypeMismatch => "Unexpected type of property",
            PropertyError::UnknownDocumentType => "Unknown document type",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PropertyError::Iri(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<iri::ParseError> for PropertyError {
    fn from(e: iri::ParseError) -> Self {
        PropertyError::Iri(e)
    }
}
