//! ActivityPub document related stuff.

pub use serde_json::Value as JsonValue;
pub use serde_json::Number as JsonNumber;

pub mod vocabulary;
pub mod view;


/// A type for a JSON map object.
pub type JsonObject = ::serde_json::Map<String, JsonValue>;
