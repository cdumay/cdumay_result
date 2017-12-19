#![deny(warnings)]
extern crate uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "cdumay-rest-client")]
extern crate cdumay_rest_client;

use std::fmt;
use std::ops::Add;
use serde_json::{Map, Value};

pub fn random_uuid() -> String {
    uuid::Uuid::new_v4().hyphenated().to_string()
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionResult {
    uuid: String,
    retcode: u16,
    stdout: String,
    stderr: String,
    retval: Map<String, Value>
}

impl ExecutionResult {
    pub fn new(retcode: Option<u16>, stdout: Option<String>, stderr: Option<String>, retval: Option<Map<String, Value>>, uuid: Option<String>) -> ExecutionResult {
        ExecutionResult {
            uuid: uuid.unwrap_or(random_uuid()),
            retcode: retcode.unwrap_or(0),
            stdout: stdout.unwrap_or(String::new()),
            stderr: stderr.unwrap_or(String::new()),
            retval: retval.unwrap_or(Map::new()),
        }
    }
    pub fn print(&mut self, data: String) {
        self.stdout += &format!("{}\n", data);
    }
    pub fn print_err(&mut self, data: String) {
        self.stderr += &format!("{}\n", data);
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

impl fmt::Debug for ExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Result<recode='{}'>", self.retcode)
    }
}

impl Add for ExecutionResult {
    type Output = ExecutionResult;
    fn add(self, other: ExecutionResult) -> ExecutionResult {
        ExecutionResult {
            uuid: self.uuid,
            retcode: if self.retcode > other.retcode { self.retcode } else { other.retcode },
            stdout: if other.stdout.len() > 0 { format!("{}\n{}", self.stdout, other.stdout) } else { format!("{}", self.stdout) },
            stderr: if other.stderr.len() > 0 { format!("{}\n{}", self.stderr, other.stderr) } else { format!("{}", self.stderr) },
            retval: {
                let mut merge = Map::new();
                for (key, value) in self.retval.into_iter() {
                    merge.insert(key, value);
                }
                for (key, value) in other.retval.into_iter() {
                    if merge.contains_key(&key) == false {
                        merge.insert(key, value);
                    }
                }
                merge
            }
        }
    }
}

#[cfg(feature = "cdumay-rest-client")]
impl From<cdumay_rest_client::exceptions::HTTPException> for ExecutionResult {
    fn from(error: cdumay_rest_client::exceptions::HTTPException) -> ExecutionResult {
        ExecutionResult {
            retcode: error.code(),
            uuid: random_uuid(),
            retval: error.extra(),
            stderr: error.message(),
            stdout: String::new(),
        }
    }
}

#[test]
fn test() {
    let err = cdumay_rest_client::exceptions::HTTPException::new(500, None, None, None);
    let res = ExecutionResult::from(err);
    println!("{:?}", res);
}