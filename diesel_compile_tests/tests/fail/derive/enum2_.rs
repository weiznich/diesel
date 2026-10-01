use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::sql_types::SqlType;
use diesel::types::Enum;

// this should fail as we don't have any discriminant
#[derive(Debug, Enum)]
#[diesel(sql_type = diesel::sql_types::Integer)]
enum Test2 {
    A,
}

// this should fail as we don't have any discriminant
#[derive(Debug, Enum)]
#[diesel(sql_type = diesel::sql_types::Blob)]
enum Test3 {
    A,
}

#[derive(SqlType, QueryId, Clone)]
#[diesel(enum_type)]
#[diesel(postgres_type(name = "Foo"))]
#[diesel(sqlite_type(name = "Integer"))]
struct SqlEnum;

#[derive(Debug, Enum)]
#[diesel(sql_type = SqlEnum)]
enum Test4 {
    A,
}

fn main() {
    let conn = &mut SqliteConnection::establish("_").unwrap();
    let pg_conn = &mut PgConnection::establish("_").unwrap();

    let _r = diesel::select(1_i32.into_sql::<diesel::sql_types::Integer>())
        .get_result::<Test2>(conn)
        //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, Test2>` is not satisfied
        .unwrap();

    let _r = diesel::select(b"abc".into_sql::<diesel::sql_types::Blob>())
        .get_result::<Test2>(conn)
        //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, Test2>` is not satisfied
        .unwrap();

    // it works with a pg connection
    let _r = diesel::select(diesel::dsl::sql::<SqlEnum>("_")).get_result::<Test4>(pg_conn);
    // it fails with a sqlite connection
    let r = diesel::select(diesel::dsl::sql::<SqlEnum>("_")).get_result::<Test4>(conn);
    //~^ ERROR: the trait bound `Test4: FromSqlRow<SqlEnum, Sqlite>` is not satisfied
}
