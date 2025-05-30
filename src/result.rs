//! Core result type and related traits for operation outcomes.
//!
//! This module provides the [`Result`] type which represents the outcome of an operation,
//! including success and error states, along with associated data and output streams.

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use serde::{Serialize, Deserialize};
use cdumay_error::Error;
use serde_value::Value;
use uuid::Uuid;

use crate::ResultBuilder;

/// Represents the outcome of an operation with associated metadata and return values.
///
/// # Fields
///
/// * `uuid` - Unique identifier for the result
/// * `retcode` - Return code (0 for success, other values for various error conditions)
/// * `stdout` - Optional standard output content
/// * `stderr` - Optional standard error content
/// * `retval` - Map of return values associated with the operation
///
/// # Examples
///
/// Creating a success result:
/// ```rust
/// use cdumay_result::ResultBuilder;
///
/// let result = ResultBuilder::default()
///     .stdout("Operation successful".into())
///     .build();
///
/// assert!(!result.is_error());
/// ```
///
/// Creating an error result from an error type:
/// ```rust
/// use cdumay_result::Result;
/// use cdumay_error_standard::Unexpected;
/// use cdumay_error::{AsError, Error};
///
/// let error = Error::from(
///     Unexpected::new()
///         .set_message("Operation failed".into())
/// );
/// let result = Result::from(error);
///
/// assert!(result.is_error());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub uuid: Uuid,
    pub retcode: u16,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub retval: BTreeMap<String, Value>,
}

impl Result {
    /// Returns true if the result represents an error state.
    ///
    /// A result is considered an error if:
    /// - The return code is >= 300 (HTTP-style error codes)
    /// - The return code is 1 (traditional Unix error code)
    pub fn is_error(&self) -> bool {
        self.retcode >= 300 || self.retcode == 1
    }
}

impl Add for &Result {
    type Output = Result;

    /// Combines two results into a new result.
    ///
    /// The combination rules are:
    /// - Uses the second result's UUID
    /// - Takes the highest return code
    /// - Concatenates stdout and stderr with newlines
    /// - Merges the return value maps
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cdumay_result::ResultBuilder;
    ///
    /// let result1 = ResultBuilder::default()
    ///     .stdout("First output".into())
    ///     .build();
    ///
    /// let result2 = ResultBuilder::default()
    ///     .stdout("Second output".into())
    ///     .build();
    ///
    /// let combined = &result1 + &result2;
    /// assert_eq!(combined.stdout, Some("First output\nSecond output".into()));
    /// ```
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
    /// Formats the result for display, showing either success or error state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cdumay_result::ResultBuilder;
    ///
    /// let success = ResultBuilder::default()
    ///     .stdout("Success".into())
    ///     .build();
    /// println!("{}", success); // Result: Ok(0, stdout: Some("Success"))
    ///
    /// let error = ResultBuilder::default()
    ///     .retcode(1)
    ///     .stderr("Failed".into())
    ///     .build();
    /// println!("{}", error); // Result: Err(1, stderr: Some("Failed"))
    /// ```
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.is_error() {
            true => write!(f, "Result: Err({}, stderr: {:?})", self.retcode, self.stderr),
            false => write!(f, "Result: Ok({}, stdout: {:?})", self.retcode, self.stdout),
        }
    }
}

impl From<Error> for Result {
    /// Creates a Result from an Error instance.
    ///
    /// The conversion:
    /// - Uses the error's code as the return code
    /// - Sets the error message as stderr
    /// - Transfers any error details to the return value map
    fn from(error: Error) -> Result {
        ResultBuilder::default()
            .retcode(error.kind.code())
            .stderr(error.message)
            .retval(error.details.unwrap_or(BTreeMap::new()))
            .build()
    }
}
