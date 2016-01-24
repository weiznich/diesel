extern crate libsqlite3_sys as ffi;
extern crate libc;

use std::ffi::{CString, CStr};
use std::{ptr, str};

use result::*;
use result::Error::DatabaseError;

pub struct RawConnection {
    pub internal_connection: *mut ffi::sqlite3,
}

impl RawConnection {
    pub fn establish(database_url: &str) -> ConnectionResult<Self> {
        let mut conn_pointer = ptr::null_mut();
        let database_url = try!(CString::new(database_url));
        let connection_status = unsafe {
            ffi::sqlite3_open(database_url.as_ptr(), &mut conn_pointer)
        };

        match connection_status {
            ffi::SQLITE_OK => Ok(RawConnection {
                internal_connection: conn_pointer,
            }),
            err_code => {
                let message = super::error_message(err_code);
                Err(ConnectionError::BadConnection(message.into()))
            }
        }
    }

    pub fn exec(&self, query: &str) -> QueryResult<()> {
        let mut err_msg = ptr::null_mut();
        let query = try!(CString::new(query));
        unsafe {
            ffi::sqlite3_exec(
                self.internal_connection,
                query.as_ptr(),
                None,
                ptr::null_mut(),
                &mut err_msg,
            );
        }

        if !err_msg.is_null() {
            let msg = unsafe {
                let bytes = CStr::from_ptr(err_msg).to_bytes();
                str::from_utf8_unchecked(bytes).into()
            };
            unsafe { ffi::sqlite3_free(err_msg as *mut libc::c_void) };
            Err(DatabaseError(msg))
        } else {
            Ok(())
        }
    }

    pub fn rows_affected_by_last_query(&self) -> usize {
        unsafe { ffi::sqlite3_changes(self.internal_connection) as usize }
    }
}

impl Drop for RawConnection {
    fn drop(&mut self) {
        let close_result = unsafe { ffi::sqlite3_close(self.internal_connection) };
        if close_result != ffi::SQLITE_OK {
            panic!("{}", super::error_message(close_result));
        }
    }
}

// fn last_error_message(conn: *mut ffi::sqlite3) -> &'static str {
//     let message_ptr = unsafe { ffi::sqlite3_errmsg(conn as *mut ffi::sqlite3) };
//     let result = unsafe { CStr::from_ptr(message_ptr) };
//     result.to_str().unwrap()
// }
