use std::ops::Add;

use cdumay_error::ErrorRepr;

use crate::ResultBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultRepr {
    pub uuid: uuid::Uuid,
    pub retcode: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    pub retval: std::collections::BTreeMap<String, serde_value::Value>,
}


impl Default for ResultRepr {
    fn default() -> ResultRepr {
        ResultRepr {
            uuid: uuid::Uuid::new_v4(),
            retcode: 0,
            stdout: None,
            stderr: None,
            retval: std::collections::BTreeMap::new(),
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

impl std::fmt::Display for ResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.is_error() {
            true => write!(f, "Result: Err({}, stderr: {:?})", self.retcode, self.stderr),
            false => write!(f, "Result: Ok({}, stdout: {:?})", self.retcode, self.stdout),
        }
    }
}

impl From<ErrorRepr> for ResultRepr {
    fn from(error: ErrorRepr) -> ResultRepr {
        let mut builder = ResultBuilder::default()
            .retcode(error.code)
            .stderr(error.message.clone());
        if let Some(data) = error.extra {
            builder.retval = data.clone();
        }
        builder.build()
    }
}
