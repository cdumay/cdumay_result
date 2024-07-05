# cdumay_result ![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue) [![cdumay_result on crates.io](https://img.shields.io/crates/v/cdumay_result)](https://crates.io/crates/cdumay_result) [![cdumay_result on docs.rs](https://docs.rs/cdumay_result/badge.svg)](https://docs.rs/cdumay_result) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_result)

cdumay_result is a basic library used to standardize result and serialize them using [serde][__link0].

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
cdumay_result = "0.3"
serde_json = "1.0"
```

*main.rs*:

```rust
extern crate cdumay_result;
extern crate serde_json;

use cdumay_result::{ResultBuilder, JsonResult};
use std::collections::BTreeMap;
use serde_json::Value;

fn main() {
    let result = ResultBuilder::default()
        .stdout("A useful result !".into())
        .retval({
            let mut values = BTreeMap::new();
            values.insert("Hello".into(), Value::String("World".into()));
            values
        })
        .build();
    println!("{}", serde_json::to_string_pretty(&JsonResult::from(result)).unwrap());
}
```

*Output*:

```json
{
  "uuid": "166d5744-b159-48b5-b8c6-9242abd8ebfb",
  "retcode": 0,
  "stdout": "A useful result !",
  "retval": {
    "Hello": "World"
  }
}
```


 [__link0]: https://docs.serde.rs/serde/
