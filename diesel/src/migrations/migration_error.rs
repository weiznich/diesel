mod migration_error{
    use std::io;
    use std::path::PathBuf;
    
    error_chain!{
        types {
            MigrationError, MigrationErrorKind, MigrationChainErr, MigrationResult;
        }

        links {
        }

        foreign_links {
            io::Error, IoError;
        }

        errors {
            MigrationDirectoryNotFound {
                description("Unable to find migrations directory in this directory or any parent directories.")
            }
            UnknownMigrationFormat(path: PathBuf) {
                description("Invalid migration directory, the directory's name should be <timestamp>_<name_of_migration>, and it should only contain up.sql and down.sql.")
            }
            UnknownMigrationVersion(message: String) {
                description("Unable to find migration version to revert in the migrations directory.")
            }
        }
    }


    impl PartialEq for MigrationErrorKind {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    &MigrationErrorKind::MigrationDirectoryNotFound,
                    &MigrationErrorKind::MigrationDirectoryNotFound,
                ) => true,
                (
                    &MigrationErrorKind::UnknownMigrationFormat(ref p1),
                    &MigrationErrorKind::UnknownMigrationFormat(ref p2),
                ) => p1 == p2,
                _ => false
            }
        }
    }
}

mod run_migration_error{
    use std::io;
    use result::{self, TransactionError};
    error_chain!{
        types {
            RunMigrationsError, RunMigrationsErrorKind, RunMigrationsChainErr, RunMigrationsResult;
        }

        links {
            super::migration_error::MigrationError, super::migration_error::MigrationErrorKind, MigrationError;
            result::Error, result::ErrorKind, QueryError;
        }

        foreign_links {
            io::Error, IoError;
        }

        errors {
        }
    }


    impl From<TransactionError<RunMigrationsError>> for RunMigrationsError {
        fn from(e: TransactionError<RunMigrationsError>) -> Self {
            use result::TransactionError::*;
            match e {
                CouldntCreateTransaction(e) => RunMigrationsError::from(e),
                UserReturnedError(e) => e,
            }
        }
    }
}

pub use self::run_migration_error::{RunMigrationsErrorKind,
                                    RunMigrationsError,
                                    RunMigrationsResult};
pub use self::migration_error::{MigrationError,
                                MigrationErrorKind,
                                MigrationResult};
