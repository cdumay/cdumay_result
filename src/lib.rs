//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_result on crates.io](https://img.shields.io/crates/v/cdumay_result)](https://crates.io/crates/cdumay_result)
//! [![cdumay_result on docs.rs](https://docs.rs/cdumay_result/badge.svg)](https://docs.rs/cdumay_result)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_result)
//!
//! A Rust library for standardizing operation results and their serialization using [serde](https://docs.serde.rs/serde/).
//! This crate provides a flexible and consistent way to handle operation results, including success and error cases,
//! with support for structured data, stdout/stderr outputs, and serialization.
//!
//! ## Features
//!
//! - Structured result type with support for return values and output streams
//! - UUID generation for result tracking
//! - Integration with error handling systems
//! - Builder pattern for easy result construction
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cdumay_result = "1.0"
//! serde_json = "1.0"  # For JSON serialization
//! cdumay_error = "1.0"  # For error handling
//! cdumay_error_standard = "1.0"  # For error example
//! ```
//!
//! ## Basic Usage
//!
//! Here's a simple example of creating and using a successful result:
//!
//! ```rust
//! use cdumay_result::{ResultBuilder, Result};
//! use std::collections::BTreeMap;
//! use serde_value::Value;
//!
//! // Creating a successful result with data
//! let result = ResultBuilder::default()
//!     .stdout("Operation completed successfully".into())
//!     .retval({
//!         let mut values = BTreeMap::new();
//!         values.insert("status".into(), Value::String("completed".into()));
//!         values.insert("count".into(), Value::U8(42.into()));
//!         values
//!     })
//!     .build();
//!
//! // Serialize to JSON
//! println!("{}", serde_json::to_string_pretty(&result).unwrap());
//! ```
//!
//! ## Error Handling
//!
//! Example using error handling with `cdumay_error_standard`:
//!
//! ```rust
//! use cdumay_result::Result;
//! use cdumay_error_standard::Unexpected;
//! use cdumay_error::{AsError, Error};
//! use serde_value::Value;
//!
//! fn process_data() -> Result {
//!     // Simulate an error condition
//!     let error = Unexpected::new()
//!         .set_message("Failed to process data".into())
//!         .set_details({
//!             let mut details = std::collections::BTreeMap::new();
//!             details.insert(
//!                 "error_type".into(),
//!                 Value::String("processing_error".into())
//!             );
//!             details
//!         });
//!
//!     // Convert error into Result
//!     Result::from(Error::from(error))
//! }
//! ```
//!
//! ## Custom Result Builder
//!
//! Example of using the builder pattern with custom data:
//!
//! ```rust
//! use cdumay_result::ResultBuilder;
//! use std::collections::BTreeMap;
//! use std::fmt::format;
//! use serde_value::Value;
//! use uuid::Uuid;
//!
//! // Create a custom result with specific UUID
//! let result = ResultBuilder::default()
//!     .uuid( Uuid::parse_str("da1c7a76-33a8-448e-9ada-3a1b17c12279").unwrap())
//!     .stdout("Custom operation result".into())
//!     .retval({
//!         let mut data = BTreeMap::new();
//!         data.insert("custom_field".into(), Value::String("custom_value".into()));
//!         data
//!     })
//!     .build();
//!
//! assert_eq!(format!("{}", result.uuid), "da1c7a76-33a8-448e-9ada-3a1b17c12279".to_string());
//! ```
//!
//! ## JSON Output Format
//!
//! The JSON output format for successful results looks like this:
//!
//! ```json
//! {
//!   "uuid": "550e8400-e29b-41d4-a716-446655440000",
//!   "retcode": 0,
//!   "stdout": "Operation completed successfully",
//!   "stderr": null,
//!   "retval": {
//!     "status": "completed",
//!     "count": 42
//!   }
//! }
//! ```
//!
//! For error results:
//!
//! ```json
//! {
//!   "uuid": "550e8400-e29b-41d4-a716-446655440000",
//!   "retcode": 1,
//!   "stdout": null,
//!   "stderr": "Failed to process data",
//!   "retval": {
//!     "error_type": "processing_error"
//!   }
//! }
//! ```

pub use builder::ResultBuilder;
pub use result::Result;

mod result;
mod builder;