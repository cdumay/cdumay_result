# cdumay_result

[![Build Status](https://travis-ci.org/cdumay/rust-cdumay_result.svg?branch=master)](https://travis-ci.org/cdumay/rust-cdumay_result)
[![Latest version](https://img.shields.io/crates/v/cdumay_result.svg)](https://crates.io/crates/cdumay_result)
[![Documentation](https://docs.rs/cdumay_result/badge.svg)](https://docs.rs/cdumay_result)
![License](https://img.shields.io/crates/l/cdumay_result.svg)

cdumay_result is a basic library used to standardize result and serialize them using [serde](https://docs.serde.rs/serde/).

## Quickstart

_Cargo.toml_:
```toml
[dependencies]
cdumay_result = "0.1"
serde_json = "1.0"
serde-value = "0.6"
```

_main.rs_:

```rust
extern crate cdumay_result;
extern crate serde_json;
extern crate serde_value;

fn main() {
    use cdumay_result::ResultReprBuilder;
    use std::collections::HashMap;
    use serde_value::Value;

    let result = ResultReprBuilder::new(None, None)
        .stdout("A useful result !".to_string())
        .retval({
            let mut retval = HashMap::new();
            retval.insert("one".to_string(), Value::I32(1));
            retval
        })
        .build();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
```
_Output_:
```json
{
  "uuid": "166d5744-b159-48b5-b8c6-9242abd8ebfb",
  "retcode": 0,
  "stdout": "A useful result !",
  "retval": {
    "one": 1
  }
}
```
## Project Links

- Issues: https://github.com/cdumay/rust-cdumay_result/issues
- Documentation: https://docs.rs/cdumay_result

License: MIT
