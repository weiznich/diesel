extern crate diesel;

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
        user_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(users, posts);

fn main() {
    let stuff = users::table.select((posts::id, posts::user_id));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
    let stuff = users::table.select((posts::id, users::name));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
}
