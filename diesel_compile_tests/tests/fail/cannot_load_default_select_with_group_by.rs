extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

fn main() {
    let mut conn = PgConnection::establish("").unwrap();
    let _ = users::table
        .group_by(users::name)
        .load::<(i32, String)>(&mut conn);
    //~^ ERROR: the trait bound `SelectStatement<_, _, _, _, _, _, _>: LoadQuery<'_, _, _>` is not satisfied
}
