//! Representation of migrations

mod errors;
pub use self::errors::{MigrationError, RunMigrationsError};

use connection::{SimpleConnection, Connection};
use deserialize::FromSql;
use expression::bound::Bound;
use expression_methods::ExpressionMethods;
use insertable::ColumnInsertValue;
use query_builder::{InsertStatement, ValuesClause};
use query_dsl::load_dsl::ExecuteDsl;
use result::QueryResult;
use sql_types::Text;
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;
use {QueryDsl, RunQueryDsl};

/// Represents a migration that interacts with diesel
pub trait Migration {
    /// Get the migration version
    fn version(&self) -> &str;
    /// Apply this migration
    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError>;
    /// Revert this migration
    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError>;
    /// Get the migration file path
    fn file_path(&self) -> Option<&Path> {
        None
    }
}

impl Migration for Box<Migration> {
    fn version(&self) -> &str {
        (&**self).version()
    }

    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).run(conn)
    }

    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).revert(conn)
    }
    fn file_path(&self) -> Option<&Path> {
        (&**self).file_path()
    }
}

impl<'a> Migration for &'a Migration {
    fn version(&self) -> &str {
        (&**self).version()
    }

    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).run(conn)
    }

    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).revert(conn)
    }
    fn file_path(&self) -> Option<&Path> {
        (&**self).file_path()
    }
}


/// A connection which can be passed to the migration methods. This exists only
/// to wrap up some constraints which are meant to hold for *all* connections.
/// This trait will go away at some point in the future. Any Diesel connection
/// should be useable where this trait is required.
#[doc(hidden)]
pub trait MigrationConnection: Connection
where
    Self: Connection,
    String: FromSql<Text, Self::Backend>,
for<'a> InsertStatement<__diesel_schema_migrations::table, ValuesClause<ColumnInsertValue<__diesel_schema_migrations::version, &'a Bound<Text, &'a str>>, __diesel_schema_migrations::table>>: ExecuteDsl<Self>,
{
    const CREATE_MIGRATIONS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS __diesel_schema_migrations (\
         version VARCHAR(50) PRIMARY KEY NOT NULL,\
         run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP\
         )";

    #[doc(hidden)]
    fn previously_run_migration_versions(&self) -> QueryResult<HashSet<String>> {
        __diesel_schema_migrations::table
            .select(__diesel_schema_migrations::version)
            .load(self)
            .map(FromIterator::from_iter)
    }

    #[doc(hidden)]
    fn latest_run_migration_version(&self) -> QueryResult<Option<String>> {
        use dsl::max;
        __diesel_schema_migrations::table
            .select(max(__diesel_schema_migrations::version))
            .first(self)
    }

    #[doc(hidden)]
    fn insert_new_migration(&self, version: &str) -> QueryResult<()> {
        try!(
            ::insert_into(__diesel_schema_migrations::table)
                .values(&__diesel_schema_migrations::version.eq(version))
                .execute(self)
        );
        Ok(())
    }
}

use self::schema::__diesel_schema_migrations;

#[doc(hidden)]
pub mod schema {
    table! {
        __diesel_schema_migrations (version) {
            version -> VarChar,
            run_on -> Timestamp,
        }
    }
}
