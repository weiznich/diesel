#[doc(hidden)]
pub mod raw;

use backend::Sqlite;
use query_builder::*;
use query_source::*;
use result::*;
use self::raw::*;
use super::{SimpleConnection, Connection};
use types::HasSqlType;

pub struct SqliteConnection {
    raw_connection: RawConnection,
}

impl SimpleConnection for SqliteConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.raw_connection.exec(query)
    }
}

impl Connection for SqliteConnection {
    type Backend = Sqlite;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        RawConnection::establish(database_url).map(|conn| {
            SqliteConnection {
                raw_connection: conn,
            }
        })
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        try!(self.batch_execute(query));
        Ok(self.raw_connection.rows_affected_by_last_query())
    }

    fn query_all<'a, T, U: 'a>(&self, _source: T) -> QueryResult<Box<Iterator<Item=U> + 'a>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend>,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend>,
    {
        unimplemented!()
    }

    fn execute_returning_count<T>(&self, _source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend>,
    {
        unimplemented!()
    }

    fn silence_notices<F: FnOnce() -> T, T>(&self, _f: F) -> T {
        unimplemented!()
    }

    fn begin_transaction(&self) -> QueryResult<()>{
        unimplemented!()
    }

    fn rollback_transaction(&self) -> QueryResult<()> {
        unimplemented!()
    }

    fn commit_transaction(&self) -> QueryResult<()> {
        unimplemented!()
    }

    fn get_transaction_depth(&self) -> i32 {
        unimplemented!()
    }
}
