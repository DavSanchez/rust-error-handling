use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SomeModuleError {
    #[error("Encountered error A with value \"{0}\" :: String")]
    OpA(String),
    #[error("Encountered error B with value `{0}` :: usize")]
    OpB(usize),
    #[error("Encountered error C: {op}")]
    OpC {
        #[from]
        #[backtrace]
        op: OpC,
    },
}

#[derive(Debug, Error)]
#[error("Values a: {a}, b: {b}")]
pub struct OpC {
    pub a: String,
    pub b: usize,
    pub backtrace: Backtrace,
}
