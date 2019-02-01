#[deny(warnings)]
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
#[cfg(feature = "cdumay-error")]
extern crate cdumay_error;

#[macro_use]
extern crate serde_derive;

use cdumay_error::ErrorProperties;
use std::ops::Add;

pub trait ResultProps {
    fn uuid(&self) -> &uuid::Uuid;
    fn uuid_mut(&mut self) -> &mut uuid::Uuid;
    fn retcode(&self) -> &u16;
    fn retcode_mut(&mut self) -> &mut u16;
    fn stdout(&self) -> &Option<String>;
    fn stdout_mut(&mut self) -> &mut Option<String>;
    fn stderr(&self) -> &Option<String>;
    fn stderr_mut(&mut self) -> &mut Option<String>;
    fn retval(&self) -> &std::collections::HashMap<String, serde_value::Value>;
    fn retval_mut(&mut self) -> &mut std::collections::HashMap<String, serde_value::Value>;

    fn is_error(&self) -> bool { *self.retcode() > 200 }
    fn search_value(&self, key: &str, default: Option<serde_value::Value>) -> Option<serde_value::Value> {
        match self.retval().get(key) {
            Some(data) => Some(data.clone()),
            None => default
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultRepr {
    uuid: uuid::Uuid,
    retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
    retval: std::collections::HashMap<String, serde_value::Value>,
}


impl Default for ResultRepr {
    fn default() -> ResultRepr {
        ResultRepr {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: std::collections::HashMap::new(),
        }
    }
}


impl ResultProps for ResultRepr {
    fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    fn retcode(&self) -> &u16 { &self.retcode }
    fn retcode_mut(&mut self) -> &mut u16 { &mut self.retcode }
    fn stdout(&self) -> &Option<String> { &self.stdout }
    fn stdout_mut(&mut self) -> &mut Option<String> { &mut self.stdout }
    fn stderr(&self) -> &Option<String> { &self.stderr }
    fn stderr_mut(&mut self) -> &mut Option<String> { &mut self.stderr }
    fn retval(&self) -> &std::collections::HashMap<String, serde_value::Value> { &self.retval }
    fn retval_mut(&mut self) -> &mut std::collections::HashMap<String, serde_value::Value> { &mut self.retval }
}

impl Add for &ResultRepr {
    type Output = ResultRepr;

    fn add(self, other: &ResultRepr) -> ResultRepr {
        let mut res = ResultRepr::default();

        *res.stdout_mut() = match (self.stdout(), other.stdout()) {
            (None, None) => None,
            (Some(ref data), None) | (None, Some(ref data)) => Some(data.clone()),
            (Some(ref data1), Some(ref data2)) => Some(format!("{}\n{}", data1, data2))
        };

        *res.stderr_mut() = match (self.stderr(), other.stderr()) {
            (None, None) => None,
            (Some(ref data), None) | (None, Some(ref data)) => Some(data.clone()),
            (Some(ref data1), Some(ref data2)) => Some(format!("{}\n{}", data1, data2))
        };

        for attr in &[&self, &other] {
            for (k, v) in attr.retval().iter() {
                res.retval_mut().insert(k.clone(), v.clone());
            }
        }
        *res.retcode_mut() = match self.retcode() > other.retcode() {
            true => *self.retcode(),
            false => *other.retcode()
        };
        res
    }
}

impl std::fmt::Display for ResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.is_error() {
            true => write!(f, "Result: Err({}, stderr: {:?})", self.retcode(), self.stderr()),
            false => write!(f, "Result: Ok({}, stdout: {:?})", self.retcode(), self.stdout()),
        }
    }
}

pub struct ResultReprBuilder {
    uuid: uuid::Uuid,
    retcode: u16,
    stdout: Option<String>,
    stderr: Option<String>,
    retval: Option<std::collections::HashMap<String, serde_value::Value>>,
}

impl ResultReprBuilder {
    pub fn new(uuid: Option<uuid::Uuid>, retcode: Option<u16>) -> ResultReprBuilder {
        ResultReprBuilder {
            uuid: uuid.unwrap_or(uuid::Uuid::new_v4()),
            retcode: retcode.unwrap_or(0),
            stdout: None,
            stderr: None,
            retval: None,
        }
    }
    pub fn stdout(mut self, stdout: String) -> ResultReprBuilder {
        self.stdout = Some(stdout);
        self
    }
    pub fn stderr(mut self, stderr: String) -> ResultReprBuilder {
        self.stderr = Some(stderr);
        self
    }
    pub fn retval(mut self, retval: std::collections::HashMap<String, serde_value::Value>) -> ResultReprBuilder {
        self.retval = Some(retval);
        self
    }
    pub fn build(self) -> ResultRepr {
        ResultRepr {
            uuid: self.uuid,
            retcode: self.retcode,
            stdout: self.stdout,
            stderr: self.stderr,
            retval: self.retval.unwrap_or(std::collections::HashMap::new()),
        }
    }
}


#[cfg(feature = "cdumay-error")]
use cdumay_error::ErrorRepr;

#[cfg(feature = "cdumay-error")]
impl From<ErrorRepr> for ResultRepr {
    fn from(error: ErrorRepr) -> ResultRepr {
        let mut res = ResultRepr::default();
        *res.retcode_mut() = *error.code();
        *res.stderr_mut() = Some(error.message().clone());
        if let Some(data) = error.extra() {
            *res.retval_mut() = data.clone();
        }
        res
    }
}