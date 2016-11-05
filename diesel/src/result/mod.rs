//! Error types and `Result` wrappers

/// Generic errors
mod error;
pub use self::error::{Error, ErrorKind, DatabaseErrorKind, ConnectionError, ConnectionErrorKind};

/// Database error information
mod database_error_information;
pub use self::database_error_information::DatabaseErrorInformation;

/// Transaction specific errors
mod transaction_error;
pub use self::transaction_error::TransactionError;

/// Trait to handle errors that actually mean "optional value" (for `NotFound`)
mod optional;
pub use self::optional::OptionalExtension;

pub use self::error::QueryResult;

/// A result with a `diesel::result::ConnectionError`
pub use self::error::ConnectionResult;

/// A result with a `diesel::result::TransactionErrorError`
pub type TransactionResult<T, E> = Result<T, TransactionError<E>>;


#[cfg(test)]
#[allow(warnings)]
fn error_impls_send() {
    let err: Error = unimplemented!();
    let x: &Send = &err;
}
