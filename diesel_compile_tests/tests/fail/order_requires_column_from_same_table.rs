extern crate diesel;

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

allow_tables_to_appear_in_same_query!(users, posts);

fn main() {
    let source = users::table.order(posts::id);
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: OrderDsl<id>` is not satisfied
}
