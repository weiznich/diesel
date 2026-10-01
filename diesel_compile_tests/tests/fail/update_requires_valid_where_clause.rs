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
    // Sanity check: Valid update
    update(users::table).filter(users::id.eq(1));

    update(users::table.filter(posts::id.eq(1)));
    //~^ ERROR: type mismatch resolving `<table as AppearsInFromClause<table>>::Count == Once`
    //~| ERROR: type mismatch resolving `<table as AppearsInFromClause<table>>::Count == Once`

    update(users::table).filter(posts::id.eq(1));
    //~^ ERROR: the trait bound `UpdateStatement<table, _>: FilterDsl<_>` is not satisfied

    update(users::table)
        .set(users::id.eq(1))
        .filter(posts::id.eq(1));
    //~^ ERROR: the trait bound `UpdateStatement<table, _, _>: FilterDsl<_>` is not satisfied
}
