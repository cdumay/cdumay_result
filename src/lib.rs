#[deny(warnings)]
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
#[cfg(feature = "cdumay-error")]
extern crate cdumay_error;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "cdumay-error")]
use std::ops::Add;

pub trait ResultGetters {
    fn uuid(&self) -> &uuid::Uuid;
    fn retcode(&self) -> &u16;
    fn stdout(&self) -> &Option<String>;
    fn stderr(&self) -> &Option<String>;
    fn retval(&self) -> &std::collections::HashMap<String, serde_value::Value>;

    fn is_error(&self) -> bool { *self.retcode() > 200 }
    fn search_value(&self, key: &str, default: Option<serde_value::Value>) -> Option<serde_value::Value> {
        match self.retval().get(key) {
            Some(data) => Some(data.clone()),
            None => default
        }
    }
}

pub trait ResultSetters {
    fn uuid_mut(&mut self) -> &mut uuid::Uuid;
    fn retcode_mut(&mut self) -> &mut u16;
    fn stdout_mut(&mut self) -> &mut Option<String>;
    fn stderr_mut(&mut self) -> &mut Option<String>;
    fn retval_mut(&mut self) -> &mut std::collections::HashMap<String, serde_value::Value>;
}

pub trait ResultBuilder {
    fn new(uuid: Option<uuid::Uuid>, retcode: Option<u16>) -> Self;
    fn set_retcode(self, retcode: u16) -> Self;
    fn set_stdout(self, stdout: String) -> Self;
    fn set_stderr(self, stderr: String) -> Self;
    fn set_retval(self, retval: std::collections::HashMap<String, serde_value::Value>) -> Self;
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


impl ResultGetters for ResultRepr {
    fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    fn retcode(&self) -> &u16 { &self.retcode }
    fn stdout(&self) -> &Option<String> { &self.stdout }
    fn stderr(&self) -> &Option<String> { &self.stderr }
    fn retval(&self) -> &std::collections::HashMap<String, serde_value::Value> { &self.retval }
}

impl ResultSetters for ResultRepr {
    fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    fn retcode_mut(&mut self) -> &mut u16 { &mut self.retcode }
    fn stdout_mut(&mut self) -> &mut Option<String> { &mut self.stdout }
    fn stderr_mut(&mut self) -> &mut Option<String> { &mut self.stderr }
    fn retval_mut(&mut self) -> &mut std::collections::HashMap<String, serde_value::Value> { &mut self.retval }
}

impl ResultBuilder for ResultRepr {
    fn new(uuid: Option<uuid::Uuid>, retcode: Option<u16>) -> ResultRepr {
        ResultRepr {
            uuid: uuid.unwrap_or(uuid::Uuid::new_v4()),
            retcode: retcode.unwrap_or(0),
            stdout: None,
            stderr: None,
            retval: std::collections::HashMap::new(),
        }
    }
    fn set_retcode(mut self, retcode: u16) -> ResultRepr {
        self.retcode = retcode;
        self
    }
    fn set_stdout(mut self, stdout: String) -> ResultRepr {
        self.stdout = Some(stdout);
        self
    }
    fn set_stderr(mut self, stderr: String) -> ResultRepr {
        self.stderr = Some(stderr);
        self
    }
    fn set_retval(mut self, retval: std::collections::HashMap<String, serde_value::Value>) -> ResultRepr {
        self.retval = retval;
        self
    }
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

#[cfg(feature = "cdumay-error")]
use cdumay_error::repr::ErrorRepr;
#[cfg(feature = "cdumay-error")]
use cdumay_error::ErrorGetters;

#[cfg(feature = "cdumay-error")]
impl From<ErrorRepr> for ResultRepr {
    fn from(error: ErrorRepr) -> ResultRepr {
        let mut res = ResultRepr::default()
            .set_retcode(*error.code())
            .set_stderr(error.message().clone());

        if let Some(data) = error.extra() {
            *res.retval_mut() = data.clone();
        }
        res
    }
}