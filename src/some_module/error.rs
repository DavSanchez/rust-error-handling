use thiserror::Error;

#[derive(Debug, Error)]
pub enum SomeModuleError {
    #[error("Encountered error A with value \"{0}\" :: String")]
    OpA(String),
    #[error("Encountered error B with value `{0}` :: usize")]
    OpB(usize),
    #[error("Encountered error C with values \"{0}\" :: String and `{1}` :: usize")]
    OpC(String, usize),
}
