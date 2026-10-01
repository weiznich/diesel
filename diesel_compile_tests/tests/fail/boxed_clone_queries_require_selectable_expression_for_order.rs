extern crate diesel;

use diesel::pg::Pg;
use diesel::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

table! {
    posts {
        id -> Integer,
        title -> VarChar,
    }
}

allow_tables_to_appear_in_same_query!(users, posts);

fn main() {
    users::table.into_boxed_clone::<Pg>().order(posts::title.desc());
    //~^ ERROR: the trait bound `BoxedCloneSelectStatement<'_, _, _, Pg>: OrderDsl<Desc<title>>` is not satisfied
}
