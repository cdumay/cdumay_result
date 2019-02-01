# cdumay_result

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A library to serialize and deserialize result using JSON.

## Quickstart

```toml
[dependencies]
cdumay_result = { git = "https://github.com/cdumay/cdumay-result-rs" }
serde_json = "1.0"
serde-value = "0.5"
```

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

## Features

- **cdumay-error**: Implement the `From` trait on `ErrorRepr`.

```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/cdumay-errors-rs" , features = ["http"] }
cdumay_result = { git = "https://github.com/cdumay/cdumay-result-rs", features = ["cdumay-error"]}
serde_json = "1.0"
serde-value = "0.5"
```

```rust
extern crate cdumay_error;
extern crate cdumay_result;
extern crate serde_json;
extern crate serde_value;

fn main() {
    use cdumay_error::ErrorReprBuilder;
    use cdumay_error::http::HttpErrors;
    use cdumay_result::ResultRepr;
    use serde_value::Value;
    use std::collections::HashMap;

    let err = ErrorReprBuilder::new(HttpErrors::NOT_FOUND)
        .extra({
            let mut extra = HashMap::new();
            extra.insert("url".to_string(), Value::String("https://www.example.com/cdumay".to_string()));
            extra
        })
        .message("The requested resource could not be found but may be available in the future.".to_string())
        .build();

    println!("{}", serde_json::to_string_pretty(&ResultRepr::from(err)).unwrap());
}
```

```json
{
  "uuid": "d55571a2-426f-45c9-b4b6-ec95cc617528",
  "retcode": 404,
  "stderr": "The requested resource could not be found but may be available in the future.",
  "retval": {
    "url": "https://www.example.com/cdumay"
  }
}
```

## Project Links

- Issues: https://github.com/cdumay/cdumay-result-rs/issues
- Documentation: not available yet
