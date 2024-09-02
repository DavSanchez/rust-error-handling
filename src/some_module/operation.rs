use super::error::SomeModuleError;

pub fn try_op(a: Option<String>, b: Option<usize>) -> Result<(), SomeModuleError> {
    match (a, b) {
        (Some(a), Some(b)) => Err(SomeModuleError::OpC(a, b)),
        (Some(a), None) => Err(SomeModuleError::OpA(a)),
        (None, Some(b)) => Err(SomeModuleError::OpB(b)),
        (None, None) => Ok(()),
    }
}
