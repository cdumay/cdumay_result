use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::Add;

use cdumay_error::Error;
use cdumay_core::{Uuid, Value};

use crate::ResultBuilder;

pub trait IsError {
    fn is_error(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Result {
    pub uuid: Uuid,
    pub retcode: u16,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub retval: BTreeMap<String, Value>,
}

impl IsError for Result {
    fn is_error(&self) -> bool {
        self.retcode >= 300 || self.retcode == 1
    }
}

impl Add for &Result {
    type Output = Result;

    fn add(self, other: &Result) -> Result {
        ResultBuilder::default()
            .uuid(other.uuid)
            .retcode(
                match self.retcode > other.retcode {
                    true => self.retcode,
                    false => other.retcode
                }
            )
            .stdout(
                match (self.stdout.clone(), other.stdout.clone()) {
                    (None, None) => String::new(),
                    (Some(ref data), None) | (None, Some(ref data)) => data.to_string(),
                    (Some(ref data1), Some(ref data2)) => format!("{}\n{}", data1, data2)
                })
            .stderr(
                match (self.stderr.clone(), other.stderr.clone()) {
                    (None, None) => String::new(),
                    (Some(ref data), None) | (None, Some(ref data)) => data.to_string(),
                    (Some(ref data1), Some(ref data2)) => format!("{}\n{}", data1, data2)
                }
            )
            .retval({
                let mut out = self.retval.clone();
                let mut out2 = other.retval.clone();
                out.append(&mut out2);
                out
            })
            .build()
    }
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.is_error() {
            true => write!(f, "Result: Err({}, stderr: {:?})", self.retcode, self.stderr),
            false => write!(f, "Result: Ok({}, stdout: {:?})", self.retcode, self.stdout),
        }
    }
}


impl From<Error> for Result {
    fn from(error: Error) -> Result {
        ResultBuilder::default()
            .retcode(error.kind.1)
            .stderr(error.message)
            .retval(error.extra.unwrap_or(BTreeMap::new()))
            .build()
    }
}

#[cfg(test)]
mod test {
    use cdumay_error::{ErrorBuilder, GenericErrors};
    use cdumay_core::{Uuid, Value};
    use crate::JsonResult;
    use super::*;

    #[test]
    fn test_error() {
        let err = ErrorBuilder::from(GenericErrors::GENERIC_ERROR)
            .extra({
                let mut extra = BTreeMap::new();
                extra.insert("context".into(), Value::String("Example".to_string()));
                extra
            })
            .build();
        let r1 = Result::from(err);
        assert_eq!(r1.is_error(), true);
        assert_eq!(r1.stderr, Some("Generic Error".to_string()))
    }

    #[test]
    fn test_add_err_ok() {
        let r1 = ResultBuilder::default()
            .retcode(1)
            .stderr("Something wrong!".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("Error".into(), Value::from("Err-46892"));
                retval
            })
            .build();
        assert_eq!(r1.is_error(), true);
        assert_eq!(format!("{}", r1), "Result: Err(1, stderr: Some(\"Something wrong!\"))".to_string());

        let r2 = ResultBuilder::default()
            .stdout("Ok !".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("Success".into(), Value::from("Yes!"));
                retval
            })
            .build();
        assert_eq!(format!("{}", r2), "Result: Ok(0, stdout: Some(\"Ok !\"))".to_string());

        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), true);
        let r4 = r2.add(&r1);
        assert_eq!(r4.is_error(), true);
    }
    #[test]
    fn test_add_stdout() {
        let r1 = ResultBuilder::default().stdout("Ok!".into()).build();
        let r2 = ResultBuilder::default().stdout("Ok!".into()).build();
        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), false);
        assert_eq!(r3.stdout, Some("Ok!\nOk!".to_string()));
        assert_eq!(r3.stderr, Some("".to_string()));
    }
    #[test]
    fn test_add_stderr() {
        let r1 = ResultBuilder::default().retcode(1).stderr("Error!".into()).build();
        let r2 = ResultBuilder::default().stderr("Error!".into()).build();
        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), true);
        assert_eq!(r3.stdout, Some("".to_string()));
        assert_eq!(r3.stderr, Some("Error!\nError!".to_string()));
    }
    #[test]
    fn test_ser() {
        let r1 = ResultBuilder::default()
            .uuid(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .stdout("Ok!".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("context".into(), Value::from("Example"));
                retval
            })
            .build();
        let out = format!("{}", serde_json::to_string(&JsonResult::from(r1)).unwrap());
        assert_eq!(out, "{\"uuid\":\"550e8400-e29b-41d4-a716-446655440000\",\"retcode\":0,\"stdout\":\"Ok!\",\"retval\":{\"context\":\"Example\"}}".to_string())
    }
}