extern crate diesel;

use diesel::pg::Pg;
use diesel::*;

table! {
    users {
        id -> Integer,
    }
}

fn main() {
    let mut connection = SqliteConnection::establish("").unwrap();
    users::table
        .into_boxed::<Pg>()
        .load::<(i32,)>(&mut connection);
    //~^ ERROR: the trait bound `BoxedSelectStatement<'_, _, _, Pg>: LoadQuery<'_, _, _>` is not satisfied
}
