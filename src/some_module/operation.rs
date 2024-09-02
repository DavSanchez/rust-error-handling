use crate::some_module::error::OpC;

use super::error::SomeModuleError;
use std::backtrace::Backtrace;

pub fn try_op(a: Option<String>, b: Option<usize>) -> Result<(), SomeModuleError> {
    match (a, b) {
        (Some(a), Some(b)) => Err(OpC {
            a,
            b,
            backtrace: Backtrace::capture(),
        }
        .into()),
        (Some(a), None) => Err(SomeModuleError::OpA(a)),
        (None, Some(b)) => Err(SomeModuleError::OpB(b)),
        (None, None) => Ok(()),
    }
}
