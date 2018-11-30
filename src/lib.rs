extern crate uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::ops::Add;

#[derive(Serialize, Clone, Debug)]
pub struct ExecResult {
    uuid: uuid::Uuid,
    retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retval: Option<serde_json::Map<String, serde_json::Value>>,
}

impl ExecResult {
    fn uuid(self) -> uuid::Uuid { self.uuid }
    fn retcode(self) -> u16 { self.retcode }
    fn stdout(self) -> Option<String> { self.stdout }
    fn stderr(self) -> Option<String> { self.stderr }
    fn retval(self) -> Option<serde_json::Map<String, serde_json::Value>> { self.retval }
    fn search_value(self, key: &str, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        match self.retval {
            Some(data) => data.get(key).cloned(),
            None => default.clone()
        }
    }
}

pub struct ExecResultBuilder {
    uuid: uuid::Uuid,
    retcode: u16,
    stdout: Option<String>,
    stderr: Option<String>,
    retval: Option<serde_json::Map<String, serde_json::Value>>,
}

impl From<ExecResultBuilder> for ExecResult {
    fn from(builder: ExecResultBuilder) -> ExecResult {
        ExecResult {
            uuid: builder.uuid,
            retcode: builder.retcode,
            stdout: builder.stdout,
            stderr: builder.stderr,
            retval: builder.retval,
        }
    }
}

impl Default for ExecResultBuilder {
    fn default() -> ExecResultBuilder {
        ExecResultBuilder {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: None,
        }
    }
}

impl ExecResultBuilder {
    pub fn uuid(mut self, uuid: uuid::Uuid) -> ExecResultBuilder {
        self.uuid = uuid;
        self
    }
    pub fn retcode(mut self, retcode: u16) -> ExecResultBuilder {
        self.retcode = retcode;
        self
    }
    pub fn retval(mut self, retval: serde_json::Map<String, serde_json::Value>) -> ExecResultBuilder {
        self.retval = Some(retval);
        self
    }
    pub fn stdout(mut self, stdout: &str) -> ExecResultBuilder {
        self.stdout = Some(stdout.to_string());
        self
    }
    pub fn stderr(mut self, stderr: &str) -> ExecResultBuilder {
        self.stderr = Some(stderr.to_string());
        self
    }
    pub fn build(self) -> ExecResult { ExecResult::from(self) }
}

impl Add<ExecResult> for ExecResult {
    type Output = ExecResult;

    fn add(self, other: ExecResult) -> <Self as Add<ExecResult>>::Output {
        let mut stdout = String::new();
        if let Some(data) = self.stdout {
            stdout.push_str(&data);
        }
        if let Some(data) = other.stdout {
            stdout.push_str(&data);
        }
        let mut stderr = String::new();
        if let Some(data) = self.stderr {
            stderr.push_str(&data);
        }
        if let Some(data) = other.stderr {
            stderr.push_str(&data);
        }
        let mut retval = serde_json::Map::new();
        if let Some(data) = self.retval {
            retval.extend(data)
        }
        if let Some(data) = other.retval {
            retval.extend(data)
        }
        ExecResultBuilder::default()
            .retcode(match self.retcode > other.retcode {
                true => self.retcode,
                false => other.retcode
            })
            .uuid(self.uuid)
            .stdout(&stdout)
            .stderr(&stderr)
            .retval(retval)
            .build()
    }
}
