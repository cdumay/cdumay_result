extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate cdumay_values;
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
    retval: HashMap<String, cdumay_values::Value>,
}

pub struct ExecResultBuilder {
    uuid: uuid::Uuid,
    retcode: u16,
    stdout: Option<String>,
    stderr: Option<String>,
    retval: HashMap<String, cdumay_values::Value>,
}

impl ExecResultBuilder {
    pub fn new() -> ExecResultBuilder {
        ExecResultBuilder {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: HashMap::new(),
        }
    }
    pub fn uuid(mut self, uuid: uuid::Uuid) -> ExecResultBuilder {
        self.uuid = uuid;
        self
    }
    pub fn retcode(mut self, retcode: u16) -> ExecResultBuilder {
        self.retcode = retcode;
        self
    }
    pub fn stdout(mut self, stdout: String) -> ExecResultBuilder {
        self.stdout = Some(stdout);
        self
    }
    pub fn stderr(mut self, stderr: String) -> ExecResultBuilder {
        self.stderr = Some(stderr);
        self
    }
    pub fn retval(mut self, retval: HashMap<String, cdumay_values::Value>) -> ExecResultBuilder {
        self.retval = retval;
        self
    }
    pub fn build(self) -> ExecResult {
        let mut result = ExecResult::default();
        *result.uuid_mut() = self.uuid;
        *result.retcode_mut() = self.retcode;
        *result.stdout_mut() = self.stdout;
        *result.stderr_mut() = self.stderr;
        *result.retval_mut() = self.retval;
        result
    }
}

impl ExecResult {
    pub fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    pub fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    pub fn retcode(&self) -> &u16 { &self.retcode }
    pub fn retcode_mut(&mut self) -> &mut u16 { &mut self.retcode }
    pub fn stdout(&self) -> &Option<String> { &self.stdout }
    pub fn stdout_mut(&mut self) -> &mut Option<String> { &mut self.stdout }
    pub fn stderr(&self) -> &Option<String> { &self.stderr }
    pub fn stderr_mut(&mut self) -> &mut Option<String> { &mut self.stderr }
    pub fn retval(&self) -> &HashMap<String, cdumay_values::Value> { &self.retval }
    pub fn retval_mut(&mut self) -> &mut HashMap<String, cdumay_values::Value> { &mut self.retval }

    pub fn is_error(&self) -> bool { self.retcode != 0 }
    pub fn search_value(&self, key: &str, default: Option<cdumay_values::Value>) -> Option<cdumay_values::Value> {
        match self.retval.get(key) {
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