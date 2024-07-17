use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use cdumay_core::{Uuid, Value};

use crate::Result;
use crate::result::IsError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonResult {
    uuid: Uuid,
    retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
    retval: BTreeMap<String, Value>,
}

impl From<Result> for JsonResult {
    fn from(value: Result) -> Self {
        JsonResult {
            uuid: value.uuid,
            retcode: value.retcode,
            stdout: value.stdout,
            stderr: value.stderr,
            retval: value.retval,
        }
    }
}

impl IsError for JsonResult {
    fn is_error(&self) -> bool {
        self.retcode >= 300 || self.retcode == 1
    }
}