use std::collections::BTreeMap;
use std::ops::Add;

use cdumay_error::ErrorRepr;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultRepr {
    pub uuid: uuid::Uuid,
    pub retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    pub retval: BTreeMap<String, serde_json::Value>,
}


impl Default for ResultRepr {
    fn default() -> ResultRepr {
        ResultRepr {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: BTreeMap::new(),
        }
    }
}

impl ResultRepr {
    pub fn is_error(&self) -> bool {
        self.retcode >= 300 || self.retcode == 1
    }
}


impl Add for &ResultRepr {
    type Output = ResultRepr;

    fn add(self, other: &ResultRepr) -> ResultRepr {
        let mut res = ResultRepr::default();
        res.stdout = match (self.stdout.clone(), other.stdout.clone()) {
            (None, None) => None,
            (Some(ref data), None) | (None, Some(ref data)) => Some(data.clone()),
            (Some(ref data1), Some(ref data2)) => Some(format!("{}\n{}", data1, data2))
        };
        res.stderr = match (self.stderr.clone(), other.stderr.clone()) {
            (None, None) => None,
            (Some(ref data), None) | (None, Some(ref data)) => Some(data.clone()),
            (Some(ref data1), Some(ref data2)) => Some(format!("{}\n{}", data1, data2))
        };
        for attr in &[&self, &other] {
            for (k, v) in attr.retval.iter() {
                res.retval.insert(k.clone(), v.clone());
            }
        }
        res.retcode = match self.retcode > other.retcode {
            true => self.retcode,
            false => other.retcode
        };
        res
    }
}

impl Display for ResultRepr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.is_error() {
            true => write!(f, "Result: Err({}, stderr: {:?})", self.retcode, self.stderr),
            false => write!(f, "Result: Ok({}, stdout: {:?})", self.retcode, self.stdout),
        }
    }
}

impl From<ErrorRepr> for ResultRepr {
    fn from(error: ErrorRepr) -> ResultRepr {
        let mut result = ResultRepr::default();
        result.retcode = error.code;
        result.stderr = Some(error.message);
        if let Some(data) = error.extra {
            result.retval = data.clone();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cdumay_error::{ErrorRepr, GenericErrors};
    use serde_json::Value;
    use uuid::Uuid;
    use ResultRepr;

    #[test]
    fn test_error() {
        let mut err = ErrorRepr::from(GenericErrors::GENERIC_ERROR);
        err.extra = Some({
            let mut extra = BTreeMap::new();
            extra.insert("context".into(), Value::String("Example".to_string()));
            extra
        });
        let r1 = ResultRepr::from(err);
        assert_eq!(r1.is_error(), true);
        assert_eq!(r1.stderr, Some("Generic Error".to_string()))
    }

    #[test]
    fn test_add_err_ok() {
        let mut r1 = ResultRepr::default();
        r1.retcode = 1;
        r1.stderr = Some("Something wrong!".to_string());
        r1.retval.insert("Error".to_string(), Value::from("Err-46892"));
        assert_eq!(r1.is_error(), true);
        assert_eq!(format!("{}", r1), "Result: Err(1, stderr: Some(\"Something wrong!\"))".to_string());

        let mut r2 = ResultRepr::default();
        r2.stdout = Some("Ok !".to_string());
        r2.retval.insert("Success".to_string(), Value::from("Yes!"));
        assert_eq!(format!("{}", r2), "Result: Ok(0, stdout: Some(\"Ok !\"))".to_string());

        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), true);
        let r4 = r2.add(&r1);
        assert_eq!(r4.is_error(), true);
    }
    #[test]
    fn test_add_ok_ok() {
        let mut r1 = ResultRepr::default();
        let mut r2 = ResultRepr::default();
        let mut r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), false);
        assert_eq!(r3.stdout, None);

        r1.stdout = Some("Ok!".to_string());
        r2.stdout = Some("Ok!".to_string());
        r3 = r1.add(&r2);
        assert_eq!(r3.stdout, Some("Ok!\nOk!".to_string()));

        r1.stderr = Some("Error!".to_string());
        r2.stderr = Some("Error!".to_string());
        r3 = r1.add(&r2);
        assert_eq!(r3.stderr, Some("Error!\nError!".to_string()));
    }
    #[test]
    fn test_ser() {
        let mut r1 = ResultRepr::default();
        r1.uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        r1.stdout = Some("Ok!".to_string());
        r1.retval.insert("context".into(), Value::String("Example".to_string()));
        let out = format!("{}", serde_json::to_string::<ResultRepr>(&r1).unwrap());
        assert_eq!(out, "{\"uuid\":\"550e8400-e29b-41d4-a716-446655440000\",\"retcode\":0,\"stdout\":\"Ok!\",\"retval\":{\"context\":\"Example\"}}".to_string())
    }
}