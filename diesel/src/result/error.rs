use ::std::error::Error as StdError;
use ::std::ffi::NulError;
use query_builder::{BuildQueryError, BuildQueryErrorKind};

error_chain!{
    types {
        Error, ErrorKind, ChainErr, QueryResult;
    }

    links {
        DatabaseError, DatabaseErrorKind, DatabaseError;
        BuildQueryError, BuildQueryErrorKind, QueryBuildError;
    }

    foreign_links {
        NulError, InvalidCString;
    }

    errors {
        NotFound {
            description("Record not found")
        }
        DeserializationError(err: Box<StdError+Send+Sync>) {
            description(err.description())
            display("Deserialization error: {}", err)
        }
        SerializationError(err: Box<StdError+Send+Sync>) {
            description(err.description())
            display("Serialization error: {}", err)
        }
        // Match against _ instead, more variants may be added in the future
        #[doc(hidden)] __Nonexhaustive{
            
        }
    }
}


impl PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        match (self, other) {
            (&ErrorKind::InvalidCString(ref a), &ErrorKind::InvalidCString(ref b)) => a == b,
            (&ErrorKind::DatabaseError(ref a), &ErrorKind::DatabaseError(ref b)) =>
                a == b,
            (&ErrorKind::NotFound, &ErrorKind::NotFound) => true,
            _ => false,
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.kind() == other.kind()
    }
}

mod database_error {
    use result::DatabaseErrorInformation;
    error_chain! {
        types {
            DatabaseError, DatabaseErrorKind, DatabaseChainErr, DatabaseQueryResult;
        }
        
        links { }
        
        foreign_links { }
        
        errors {
            UniqueViolation(information: Box<DatabaseErrorInformation+Send>) {
                description("Unique violation")
                display("Database error (Unique violation): {}", information.message())
            }
            // Match against _ instead, more variants may be added in the future
            #[doc(hidden)] __Unknown(information: Box<DatabaseErrorInformation+Send>) {
                description("Unknown")
                display("Database error (Unknown error): {}", information.message())
            }
        }
    }

    impl DatabaseErrorInformation for DatabaseErrorKind {
        /// Get original error message
        fn message(&self) -> &str {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.message()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.message()
                }
                DatabaseErrorKind::Msg(ref s) => &s,
            }
        }

        /// Get optional error details 
        fn details(&self) -> Option<&str> {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.details()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.details()
                }
                DatabaseErrorKind::Msg(ref s) => Some(&s),
            }
        }

        /// Get additional error hint, optional
        fn hint(&self) -> Option<&str> {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.hint()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.hint()
                }
                DatabaseErrorKind::Msg(ref s) => Some(&s),
            }
        }

        /// Get name of the table this error concerns
        fn table_name(&self) -> Option<&str> {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.table_name()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.table_name()
                }
                DatabaseErrorKind::Msg(ref s) => Some(&s),
            }
        }

        /// Get name of the column this error concerns
        fn column_name(&self) -> Option<&str> {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.column_name()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.column_name()
                }
                DatabaseErrorKind::Msg(ref s) => Some(&s),
            }
        } 

        /// Get name of the constraint this error concerns
        fn constraint_name(&self) -> Option<&str> {
            match *self {
                DatabaseErrorKind::UniqueViolation(ref inner) => {
                    inner.constraint_name()
                }
                DatabaseErrorKind::__Unknown(ref inner) => {
                    inner.constraint_name()
                }
                DatabaseErrorKind::Msg(ref s) => Some(&s),
            }
        }
    }

    impl PartialEq for DatabaseErrorKind {
        fn eq(&self, other: &DatabaseErrorKind) -> bool{
            match (self, other){
                (&DatabaseErrorKind::UniqueViolation(ref a),
                 &DatabaseErrorKind::UniqueViolation(ref b)) |
                (&DatabaseErrorKind::__Unknown(ref a),
                 &DatabaseErrorKind::__Unknown(ref b)) => a.message() == b.message(),
                _=> false,
            }
        }
    }
}

mod connection_error{
    use std::ffi::NulError;
    error_chain!{
        types {
            ConnectionError, ConnectionErrorKind, ConnectionChainErr, ConnectionResult;
        }
        
        links { }
        
        foreign_links {
            NulError, InvalidCString;
        }
        
        errors {
            BadConnection(message: String) {
                description("Bad connection error")
                    display("Bad connection error: {}", message)
            }
        }
    }
}

pub use self::database_error::{DatabaseErrorKind, DatabaseError};
pub use self::connection_error::{ConnectionErrorKind, ConnectionError, ConnectionResult};
