# cdumay_result

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A library to serialize and deserialize result using JSON.

## Quickstart

```toml
[dependencies]
cdumay_result = "0.1"
```

```rust
extern crate cdumay_result;
extern crate serde_json;

use cdumay_result::ExecutionResult;
use serde_json::{Map, Value, to_string_pretty};

let mut context: Map<String, Value> = Map::new();
context.insert("url".to_string(), Value::from("https://example.dumay"));

let err = ExecutionResult::new(Some(404), None, Some("Not Found".to_string()), Some(context), None);
println!("{}", to_string_pretty(&err).unwrap());
```

```json
{
  "uuid": "c2b1a262-bafb-4cea-a85a-39b064717545",
  "retcode": 404,
  "stdout": "",
  "stderr": "Not Found",
  "retval": {
    "url": "https://example.dumay"
  }
}
```

## Features

- [cdumay-errors](https://github.com/cdumay/cdumay-errors-rs): A basic library used to standardize errors and to serialize them into json using serde_json.

## Project Links

- Issues: https://github.com/cdumay/cdumay-result-rs/issues
- Documentation: 

## License

MIT License