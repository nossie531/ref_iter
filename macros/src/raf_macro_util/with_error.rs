//! Provider of [`WithError`].

use crate::raf_macro_util::SoftResult;
use syn::{Error, Result};

/// Trait object with error information.
pub trait WithError {
    /// Get error information.
    fn err(&self) -> Option<&Error>;
}

impl<T> WithError for SoftResult<T> {
    fn err(&self) -> Option<&Error> {
        SoftResult::err(self)
    }
}

impl<T> WithError for Result<T> {
    fn err(&self) -> Option<&Error> {
        self.as_ref().err()
    }
}
