use std::error::Error;

use log::{error, info};
use some_module::{error::SomeModuleError, operation};
use thiserror::Error;

mod some_module;

#[derive(Debug, Error)]
enum TopLevelError {
    #[error("Some error: {0}")]
    _SomeError(String),
    #[error("Internal module error: {0}")]
    InternalModuleError(#[from] SomeModuleError),
    #[error("Other error happened")]
    _Other,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Let's go!");

    let result = work("foo".to_string(), 42)
        .inspect_err(|e| error!("logging error ({}:{}) --> {}", file!(), line!(), e))?;

    info!("Result: {}", result);

    Ok(())
}

fn work(a: String, b: usize) -> Result<String, TopLevelError> {
    let some_a = if !a.is_empty() { Some(a) } else { None };
    let some_b = if b > 0 { Some(b) } else { None };

    operation::try_op(some_a, some_b)?;

    Ok("done".to_string())
}
