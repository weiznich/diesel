extern crate libsqlite3_sys as ffi;
extern crate libc;

use std::ffi::CString;
use std::ptr;

use result::*;
use result::Error::DatabaseError;
use super::raw::RawConnection;

pub struct Statement {
    _inner_statement: *mut ffi::sqlite3_stmt,
}

impl Statement {
    pub fn prepare(raw_connection: &RawConnection, sql: &str) -> QueryResult<Self> {
        let mut stmt = ptr::null_mut();
        let prepare_result = unsafe {
            ffi::sqlite3_prepare_v2(
                raw_connection.internal_connection,
                try!(CString::new(sql)).as_ptr(),
                sql.len() as libc::c_int,
                &mut stmt,
                &mut ptr::null(),
            )
        };

        if prepare_result != ffi::SQLITE_OK {
            Err(DatabaseError(raw_connection.error_from_code(prepare_result)))
        } else {
            Ok(Statement { _inner_statement: stmt })
        }
    }

    pub fn run(&self) -> QueryResult<()> {
        Ok(())
    }
}
