//! cdumay_result is a basic library used to standardize result and serialize them using [serde](https://docs.serde.rs/serde/).
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_result = "0.3"
//! serde_json = "1.0"
//! ```
//!
//! _main.rs_:
//!
//! ```rust
//! extern crate cdumay_result;
//! extern crate serde_json;
//!
//! use cdumay_result::{ResultBuilder, JsonResult};
//! use std::collections::BTreeMap;
//! use serde_json::Value;
//!
//! fn main() {
//!     let result = ResultBuilder::default()
//!         .stdout("A useful result !".into())
//!         .retval({
//!             let mut values = BTreeMap::new();
//!             values.insert("Hello".into(), Value::String("World".into()));
//!             values
//!         })
//!         .build();
//!     println!("{}", serde_json::to_string_pretty(&JsonResult::from(result)).unwrap());
//! }
//! ```
//! _Output_:
//! ```json
//! {
//!   "uuid": "166d5744-b159-48b5-b8c6-9242abd8ebfb",
//!   "retcode": 0,
//!   "stdout": "A useful result !",
//!   "retval": {
//!     "Hello": "World"
//!   }
//! }
//! ```

extern crate cdumay_error;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub use result::Result;
pub use jsonify::JsonResult;
pub use builder::ResultBuilder;

mod result;
mod jsonify;
mod builder;