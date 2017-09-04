//! ActivityPub implementation for Rust.
#![warn(missing_docs)]
// Allow unknown lints for clippy.
#![allow(unknown_lints)]
// `doc_markdown` by clippy says:
//
// > warning: you should put `ActivityPub` between ticks in the documentation
#![allow(doc_markdown)]

extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;
pub extern crate url;

pub use iri::{Iri, IriBuf};

pub mod document;
pub mod iri;
