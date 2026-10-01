extern crate diesel;

use diesel::pg::returning::old;
use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

fn main() {
    use self::users::dsl::*;

    let mut connection = PgConnection::establish("").unwrap();

    // `old(col)` is only meaningful inside a RETURNING clause.
    // Using it in a regular SELECT is rejected at compile time.
    users
        .select(old(name))
        //~^ ERROR: the trait bound `SelectStatement<_>: SelectDsl<Old<name>>` is not satisfied
        .load::<String>(&mut connection)
        //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, String>` is not satisfied
        .unwrap();

    // `old(col).nullable()` is also rejected outside RETURNING.
    users
        .select(old(name).nullable())
        //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
        .load::<Option<String>>(&mut connection)
        //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, Option<String>>` is not satisfied
        .unwrap();
}
