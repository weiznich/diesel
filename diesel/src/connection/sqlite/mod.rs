extern crate libsqlite3_sys as ffi;
extern crate libc;

use std::ptr;
use std::ffi::{CString, CStr};

use backend::Sqlite;
use expression::{AsExpression, NonAggregate};
use expression::helper_types::AsExpr;
use helper_types::{FindBy, Limit};
use query_builder::*;
use query_dsl::*;
use query_source::*;
use result::*;
use result::Error::DatabaseError;
use super::{SimpleConnection, Connection, PkType, FindPredicate};

pub struct SqliteConnection {
    internal_connection: *mut ffi::sqlite3,
}

impl SimpleConnection for SqliteConnection {
    fn batch_execute(&self, _query: &str) -> QueryResult<()> {
        unimplemented!()
    }
}

impl Connection for SqliteConnection {
    type Backend = Sqlite;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        let mut conn_pointer = ptr::null_mut();
        let database_url = try!(CString::new(database_url));
        let connection_status = unsafe {
            ffi::sqlite3_open(database_url.as_ptr(), &mut conn_pointer)
        };

        match connection_status {
            ffi::SQLITE_OK => Ok(SqliteConnection {
                internal_connection: conn_pointer,
            }),
            err_code => {
                let message = error_message(err_code);
                Err(ConnectionError::BadConnection(message.into()))
            }
        }
    }

    fn transaction<T, E, F>(&self, _f: F) -> TransactionResult<T, E> where
        F: FnOnce() -> Result<T, E>,
    {
        unimplemented!()
    }

    fn begin_test_transaction(&self) -> QueryResult<usize> {
        unimplemented!()
    }

    fn test_transaction<T, E, F>(&self, _f: F) -> T where
        F: FnOnce() -> Result<T, E>,
    {
        unimplemented!()
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        let mut stmt = ptr::null_mut();
        let mut unused = ptr::null();
        let query_size = query.len();
        let query = try!(CString::new(query));
        let stmt_result = unsafe {
            ffi::sqlite3_prepare_v2(
                self.internal_connection,
                query.as_ptr(),
                query_size as libc::c_int,
                &mut stmt,
                &mut unused,
            )
        };

        if stmt_result != ffi::SQLITE_OK {
            return Err(DatabaseError(last_error_message(self.internal_connection).into()));
        }

        let result_code = unsafe { ffi::sqlite3_step(stmt) };
        match result_code {
            ffi::SQLITE_DONE => Ok(0),
            _ => Err(DatabaseError(last_error_message(self.internal_connection).into())),
        }
    }

    fn query_one<T, U>(&self, _source: T) -> QueryResult<U> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend>,
        U: Queryable<T::SqlType>,
    {
        unimplemented!()
    }

    fn query_all<'a, T, U: 'a>(&self, _source: T) -> QueryResult<Box<Iterator<Item=U> + 'a>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend>,
        U: Queryable<T::SqlType>,
    {
        unimplemented!()
    }

    fn find<T, U, PK>(&self, source: T, id: PK) -> QueryResult<U> where
        T: Table + FilterDsl<FindPredicate<T, PK>>,
        FindBy<T, T::PrimaryKey, PK>: LimitDsl,
        Limit<FindBy<T, T::PrimaryKey, PK>>: QueryFragment<Self::Backend>,
        U: Queryable<<Limit<FindBy<T, T::PrimaryKey, PK>> as Query>::SqlType>,
        PK: AsExpression<PkType<T>>,
        AsExpr<PK, T::PrimaryKey>: NonAggregate,
    {
        unimplemented!()
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend>,
    {
        unimplemented!()
    }

    fn silence_notices<F: FnOnce() -> T, T>(&self, f: F) -> T {
        unimplemented!()
    }
}

fn error_message(err_code: libc::c_int) -> &'static str {
    let message_ptr = unsafe { ffi::sqlite3_errstr(err_code) };
    let result = unsafe { CStr::from_ptr(message_ptr) };
    result.to_str().unwrap()
}

fn last_error_message(conn: *mut ffi::sqlite3) -> &'static str {
    let message_ptr = unsafe { ffi::sqlite3_errmsg(conn as *mut ffi::sqlite3) };
    let result = unsafe { CStr::from_ptr(message_ptr) };
    result.to_str().unwrap()
}
