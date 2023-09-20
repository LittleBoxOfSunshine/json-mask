//! [![github]](https://github.com/LittleBoxOfSunshine/json-mask)&ensp;[![crates-io]](https://crates.io/crates/json_mask)&ensp;[![docs-rs]](https://docs.rs/json_mask)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This library provides [`mask::JsonMasker`] which accepts a [JSON Schema](https://json-schema.org/)
//! document.
//!
//! An example use case is an API where the backend generates the latest response version, but
//! applies the mask to transform the latest response into other API versions to satisfy backwards
//! compatibility.
//!
//! This is an early build where the input validation is more flexible. Based on real world usage
//! the first stable release will probably restrict to formal schema draft standards and/or allow
//! you to restrict to specific ones.
//!
//! # Examples
//! - Use generate a mask with [`from_str`] or [`from_reader`] and apply it to a document.
//!
//!   ```
//!   use json_mask::from_str;
//!   use json_mask::JsonMasker;
//!   use json_mask::ValidJsonSchema;
//!
//!   let mut document = serde_json::from_str(r#"{ "foo": 1, "bar": 2}"#).unwrap();
//!   let schema = r#"
//!   {
//!     "$schema": "http://json-schema.org/draft-04/schema",
//!     "title": "Demo Schema",
//!     "description": "Demo",
//!     "type": "object",
//!     "properties": {
//!       "foo": {
//!         "type": "integer"
//!       }
//!     }
//!   }
//!   "#;
//!
//!   let mask = from_str(schema).unwrap();
//!   let masker = JsonMasker::new(mask);
//!   masker.mask(&mut document);
//!
//!   assert_eq!(r#"{"foo":1}"#, serde_json::to_string(&document).unwrap())
//!   ```
//!

mod mask;

pub use mask::JsonMasker;
pub use mask::Mask;
pub use mask::ValidJsonSchema;
pub use mask::ParseError;
pub use mask::from_str;
pub use mask::from_reader;
