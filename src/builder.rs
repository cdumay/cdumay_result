use std::collections::BTreeMap;

use crate::repr::ResultRepr;

#[derive(Debug)]
pub struct ResultBuilder {
    pub uuid: Option<uuid::Uuid>,
    pub retcode: Option<u16>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub retval: BTreeMap<String, serde_value::Value>,
}

impl Default for ResultBuilder {
    fn default() -> ResultBuilder {
        ResultBuilder {
            uuid: None,
            retcode: None,
            stdout: None,
            stderr: None,
            retval: BTreeMap::new(),
        }
    }
}

impl ResultBuilder {
    pub fn uuid(mut self, uuid: uuid::Uuid) -> ResultBuilder {
        self.uuid = Some(uuid);
        self
    }
    pub fn retcode(mut self, retcode: u16) -> ResultBuilder {
        self.retcode = Some(retcode);
        self
    }
    pub fn stdout(mut self, stdout: String) -> ResultBuilder {
        self.stdout = Some(stdout);
        self
    }
    pub fn stderr(mut self, stderr: String) -> ResultBuilder {
        self.stderr = Some(stderr);
        self
    }
    pub fn retval(mut self, retval: BTreeMap<String, serde_value::Value>) -> ResultBuilder {
        self.retval.extend(retval);
        self
    }
    pub fn build(self) -> ResultRepr {
        ResultRepr {
            uuid: self.uuid.unwrap_or(uuid::Uuid::new_v4()),
            retcode: self.retcode.unwrap_or(0),
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
            retval: self.retval.clone(),
        }
    }
}