use super::{Error, QueryResult, ErrorKind};

/// Extension trait to handle optional values (i.e., treat `NotFound` errors as
/// missing values and not errors).
pub trait OptionalExtension<T> {
    fn optional(self) -> Result<Option<T>, Error>;
}

impl<T> OptionalExtension<T> for QueryResult<T> {
    fn optional(self) -> Result<Option<T>, Error> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(ref e) if e.kind() == &ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}
