#![feature(error_generic_member_access)]

use log::{error, info, trace};
use some_module::{error::SomeModuleError, operation};
use std::backtrace::Backtrace;
use std::error::request_ref;
use thiserror::Error;

mod some_module;

#[derive(Debug, Error)]
enum TopLevelError {
    #[error("Some error: {0}")]
    _SomeError(String),
    #[error("Internal module error: {0}")]
    InternalModuleError(
        #[from]
        #[backtrace]
        SomeModuleError,
    ),
    #[error("Other error happened")]
    _Other,
}

fn main() {
    env_logger::init();

    info!("Let's go!");

    match work("foo".to_string(), 42) {
        Ok(result) => info!("Result: {}", result),
        Err(e) => {
            error!("logging error ({}:{}) --> {}", file!(), line!(), e);

            let TopLevelError::InternalModuleError(ee) = e else {
                return;
            };
            // Enable with RUST_BACKTRACE=1
            let Some(backtrace) = request_ref::<Backtrace>(&ee) else {
                return;
            };
            // Enable with RUST_LOG=trace
            trace!("backtrace: {:#?}", backtrace);
        }
    }
}

fn work(a: String, b: usize) -> Result<String, TopLevelError> {
    let some_a = if !a.is_empty() { Some(a) } else { None };
    let some_b = if b > 0 { Some(b) } else { None };

    operation::try_op(some_a, some_b)?;

    Ok("done".to_string())
}
