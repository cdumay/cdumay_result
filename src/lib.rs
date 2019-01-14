extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
#[cfg(feature = "cdumay-error")]
extern crate cdumay_error;

#[macro_use]
extern crate serde_derive;

use std::ops::Add;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecResult {
    uuid: uuid::Uuid,
    retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
    retval: HashMap<String, serde_value::Value>,
}

pub trait ExecResultProperties {
    fn uuid(&self) -> &uuid::Uuid;
    fn uuid_mut(&mut self) -> &mut uuid::Uuid;
    fn retcode(&self) -> &u16;
    fn retcode_mut(&mut self) -> &mut u16;
    fn stdout(&self) -> &Option<String>;
    fn stdout_mut(&mut self) -> &mut Option<String>;
    fn stderr(&self) -> &Option<String>;
    fn stderr_mut(&mut self) -> &mut Option<String>;
    fn retval(&self) -> &HashMap<String, serde_value::Value>;
    fn retval_mut(&mut self) -> &mut HashMap<String, serde_value::Value>;

    fn is_error(&self) -> bool { *self.retcode() != 0 }
    fn search_value(&self, key: &str, default: Option<serde_value::Value>) -> Option<serde_value::Value> {
        match self.retval().get(key) {
            Some(data) => Some(data.clone()),
            None => default
        }
    }
}

impl Default for ExecResult {
    fn default() -> ExecResult {
        ExecResult {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: HashMap::new(),
        }
    }
}


impl ExecResultProperties for ExecResult {
    fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    fn retcode(&self) -> &u16 { &self.retcode }
    fn retcode_mut(&mut self) -> &mut u16 { &mut self.retcode }
    fn stdout(&self) -> &Option<String> { &self.stdout }
    fn stdout_mut(&mut self) -> &mut Option<String> { &mut self.stdout }
    fn stderr(&self) -> &Option<String> { &self.stderr }
    fn stderr_mut(&mut self) -> &mut Option<String> { &mut self.stderr }
    fn retval(&self) -> &HashMap<String, serde_value::Value> { &self.retval }
    fn retval_mut(&mut self) -> &mut HashMap<String, serde_value::Value> { &mut self.retval }
}

impl Add<ExecResult> for ExecResult {
    type Output = ExecResult;

    fn add(self, other: ExecResult) -> <Self as Add<ExecResult>>::Output {
        let mut res = ExecResult::default();
        let mut stdout = String::new();
        if let Some(data) = self.stdout {
            stdout.push_str(&data);
        }
        if let Some(data) = other.stdout {
            stdout.push_str(&data);
        }
        *res.stdout_mut() = Some(stdout);

        let mut stderr = String::new();
        if let Some(data) = self.stderr {
            stderr.push_str(&data);
        }
        if let Some(data) = other.stderr {
            stderr.push_str(&data);
        }
        *res.stderr_mut() = Some(stderr);

        res.retval_mut().extend(self.retval);
        res.retval_mut().extend(other.retval);

        *res.retcode_mut() = match self.retcode > other.retcode {
            true => self.retcode,
            false => other.retcode
        };
        res
    }
}

#[cfg(feature = "cdumay-error")]
impl From<cdumay_error::Error> for ExecResult {
    fn from(error: cdumay_error::Error) -> ExecResult {
        let mut res = ExecResult::default();
        *res.retcode_mut() = *error.code();
        *res.stderr_mut() = Some(error.message().clone());
        if let Some(data) = error.extra() {
            *res.retval_mut() = data.clone();
        }
        res
    }
}