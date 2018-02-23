#![deny(warnings)]
extern crate uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "cdumay-errors")]
extern crate cdumay_errors;

use std::fmt;
use serde_json::{Map, Value};

pub fn random_uuid() -> String {
    uuid::Uuid::new_v4().hyphenated().to_string()
}

pub trait BaseResult: fmt::Display {
    fn is_error(&self) -> bool {
        match self.retcode() {
            0 => true,
            _ => false
        }
    }
    fn new(retcode: Option<u16>, stdout: Option<String>, stderr: Option<String>, retval: Option<Map<String, Value>>, uuid: Option<String>) -> Self;
    fn print(&mut self, data: String);
    fn print_err(&mut self, data: String);
    fn uuid(&self) -> String;
    fn retcode(&self) -> u16;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
    fn retval(&self) -> Map<String, Value>;
    fn empty() -> Self;
    fn merge(&self, other: Self) -> Self;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionResult {
    uuid: String,
    retcode: u16,
    stdout: String,
    stderr: String,
    retval: Map<String, Value>,
}

impl BaseResult for ExecutionResult {
    fn new(retcode: Option<u16>, stdout: Option<String>, stderr: Option<String>, retval: Option<Map<String, Value>>, uuid: Option<String>) -> ExecutionResult {
        ExecutionResult {
            uuid: uuid.unwrap_or(random_uuid()),
            retcode: retcode.unwrap_or(0),
            stdout: stdout.unwrap_or(String::new()),
            stderr: stderr.unwrap_or(String::new()),
            retval: retval.unwrap_or(Map::new()),
        }
    }
    fn empty() -> ExecutionResult {
        ExecutionResult::new(None, None, None, None, None)
    }
    fn print(&mut self, data: String) {
        self.stdout += &format!("{}\n", data);
    }
    fn print_err(&mut self, data: String) {
        self.stderr += &format!("{}\n", data);
    }
    fn uuid(&self) -> String {
        self.uuid.clone()
    }
    fn retcode(&self) -> u16 {
        self.retcode
    }
    fn stdout(&self) -> String {
        self.stdout.clone()
    }
    fn stderr(&self) -> String {
        self.stderr.clone()
    }
    fn retval(&self) -> Map<String, Value> {
        self.retval.clone()
    }
    fn merge(&self, other: ExecutionResult) -> ExecutionResult {
        ExecutionResult {
            uuid: self.uuid(),
            retcode: if self.retcode() > other.retcode() { self.retcode() } else { other.retcode() },
            stdout: if other.stdout.len() > 0 { other.stdout() } else { self.stdout() },
            stderr: if other.stderr.len() > 0 { other.stderr() } else { self.stderr() },
            retval: {
                let mut merge = Map::new();
                for (key, value) in self.retval().into_iter() {
                    merge.insert(key, value);
                }
                for (key, value) in other.retval().into_iter() {
                    if merge.contains_key(&key) == false {
                        merge.insert(key, value);
                    }
                }
                merge
            },
        }
    }
}

impl fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.retcode {
            0 => write!(f, "{}", self.stdout),
            _ => write!(f, "{}", self.stderr)
        }
    }
}

#[cfg(feature = "cdumay-errors")]
impl From<cdumay_errors::Error> for ExecutionResult {
    fn from(error: cdumay_errors::Error) -> ExecutionResult {
        ExecutionResult {
            uuid: random_uuid(),
            retcode: error.code(),
            stdout: String::new(),
            stderr: error.message(),
            retval: error.extra(),
        }
    }
}