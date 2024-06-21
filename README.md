# cdumay_result ![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue) [![cdumay_result on crates.io](https://img.shields.io/crates/v/cdumay_result)](https://crates.io/crates/cdumay_result) [![cdumay_result on docs.rs](https://docs.rs/cdumay_result/badge.svg)](https://docs.rs/cdumay_result) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_result)

cdumay_result is a basic library used to standardize result and serialize them using [serde][__link0].

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
cdumay_result = "0.2"
serde_json = "1.0"
```

*main.rs*:

```rust
extern crate cdumay_result;
extern crate serde_json;

fn main() {
    use cdumay_result::ResultRepr;
    use std::collections::HashMap;

    let mut result = ResultRepr::default();
    result.stdout = Some("A useful result !".to_string());
    result.retval.insert("one".to_string(), serde_json::Value::from(1));
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
```

*Output*:

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


 [__link0]: https://docs.serde.rs/serde/
