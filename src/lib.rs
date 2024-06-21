//! cdumay_result is a basic library used to standardize result and serialize them using [serde](https://docs.serde.rs/serde/).
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_result = "0.2"
//! serde_json = "1.0"
//! ```
//!
//! _main.rs_:
//!
//! ```rust
//! extern crate cdumay_result;
//! extern crate serde_json;
//!
//! fn main() {
//!     use cdumay_result::ResultRepr;
//!     use std::collections::HashMap;
//!
//!     let mut result = ResultRepr::default();
//!     result.stdout = Some("A useful result !".to_string());
//!     result.retval.insert("one".to_string(), serde_json::Value::from(1));
//!     println!("{}", serde_json::to_string_pretty(&result).unwrap());
//! }
//! ```
//! _Output_:
//! ```json
//! {
//!   "uuid": "166d5744-b159-48b5-b8c6-9242abd8ebfb",
//!   "retcode": 0,
//!   "stdout": "A useful result !",
//!   "retval": {
//!     "one": 1
//!   }
//! }
//! ```

extern crate cdumay_error;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub use repr::ResultRepr;

mod repr;
