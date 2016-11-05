use diesel;
use diesel::prelude::*;
use diesel::result::ErrorKind::DatabaseError;
use diesel::result::Error;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use schema::*;

#[test]
fn unique_constraints_are_detected() {
    let connection = connection();
    diesel::insert(&User::new(1, "Sean")).into(users::table)
        .execute(&connection).unwrap();

    let failure = diesel::insert(&User::new(1, "Jim")).into(users::table)
        .execute(&connection).unwrap_err();
    assert_matches!(failure.kind(), &DatabaseError(UniqueViolation(_)));
}

#[test]
#[cfg(feature = "postgres")]
fn unique_constraints_report_correct_constraint_name() {
    let connection = connection();
    connection.execute("CREATE UNIQUE INDEX users_name ON users (name)").unwrap();
    diesel::insert(&User::new(1, "Sean")).into(users::table)
        .execute(&connection).unwrap();

    let failure = diesel::insert(&User::new(2, "Sean")).into(users::table)
        .execute(&connection);
    match failure {
        Err(ref e) => {
            match *e.kind(){
                DatabaseError(UniqueViolation(ref e)) => {
                    assert_eq!(Some("users"), e.table_name());
                    assert_eq!(None, e.column_name());
                    assert_eq!(Some("users_name"), e.constraint_name());       
                }
                _=> panic!("{:?} did not match Err(DatabaseError(UniqueViolation(e)))", failure),
            }
        },
        _ => panic!("{:?} did not match Err(DatabaseError(UniqueViolation(e)))", failure),
    };
}

macro_rules! try_no_coerce {
    ($e:expr) => ({
        match $e {
            Ok(e) => e,
            Err(e) => return Err(e),
        }
    })
}

#[test]
fn cached_prepared_statements_can_be_reused_after_error() {
    let connection = connection_without_transaction();
    let user = User::new(1, "Sean");
    let query = diesel::insert(&user).into(users::table);

    connection.test_transaction(|| {
        try_no_coerce!(query.execute(&connection));

        let failure = query.execute(&connection).unwrap_err();
        assert_matches!(failure.kind(), &DatabaseError(UniqueViolation(_)));
        Ok(())
    });

    connection.test_transaction(|| query.execute(&connection));
}
