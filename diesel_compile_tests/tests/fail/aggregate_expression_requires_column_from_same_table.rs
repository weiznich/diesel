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
    use diesel::dsl::*;
    let source = users::table.select(sum(posts::id));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
    let source = users::table.select(avg(posts::id));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
    let source = users::table.select(max(posts::id));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
    let source = users::table.select(min(posts::id));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: SelectDsl<_>` is not satisfied
}
