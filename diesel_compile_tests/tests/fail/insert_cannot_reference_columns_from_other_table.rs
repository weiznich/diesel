extern crate diesel;

use diesel::pg::PgConnection;
use diesel::*;

table! {
    users {
        id -> Integer,
    }
}

table! {
    posts {
        id -> Integer,
    }
}

fn main() {
    let mut conn = PgConnection::establish("").unwrap();

    insert_into(users::table).values(&posts::id.eq(1));
    //~^ ERROR: the trait bound `Eq<id, &Bound<Integer, i32>>: Insertable<table>` is not satisfied

    insert_into(users::table).values(&(posts::id.eq(1), users::id.eq(2)));
    //~^ ERROR: the trait bound `Eq<id, &Bound<Integer, i32>>: Insertable<table>` is not satisfied
    //FIXME: Bad error on the second one
}
