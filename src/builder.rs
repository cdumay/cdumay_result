use std::collections::BTreeMap;

use cdumay_core::{Uuid, Value};

use crate::Result;

pub struct ResultBuilder {
    uuid: Uuid,
    retcode: u16,
    stdout: Option<String>,
    stderr: Option<String>,
    retval: BTreeMap<String, Value>,
}

impl Default for ResultBuilder {
    fn default() -> ResultBuilder {
        ResultBuilder {
            uuid: Uuid::new_v4(),
            retcode: 0,
            stdout: Default::default(),
            stderr: Default::default(),
            retval: Default::default(),
        }
    }
}


impl ResultBuilder {
    pub fn uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = uuid;
        self
    }
    pub fn retcode(mut self, retcode: u16) -> Self {
        self.retcode = retcode;
        self
    }
    pub fn stdout(mut self, stdout: String) -> Self {
        self.stdout = Some(stdout);
        self
    }
    pub fn stderr(mut self, stderr: String) -> Self {
        self.stderr = Some(stderr);
        self
    }
    pub fn retval(mut self, retval: BTreeMap<String, Value>) -> Self {
        self.retval = retval;
        self
    }
    pub fn build(self) -> Result {
        Result {
            uuid: self.uuid,
            retcode: self.retcode,
            stdout: self.stdout,
            stderr: self.stderr,
            retval: self.retval,
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use Value;

    use crate::builder::ResultBuilder;

    #[test]
    fn test_builder() {
        let result = ResultBuilder::default()
            .stdout("Test".to_string())
            .retval({
                let mut extra = BTreeMap::new();
                extra.insert("Hello".into(), Value::String("World".to_string()));
                extra
            })
            .build();
        assert_eq!(result.stdout, Some("Test".to_string()));
        assert!(result.retval.contains_key("Hello"))
    }
}
