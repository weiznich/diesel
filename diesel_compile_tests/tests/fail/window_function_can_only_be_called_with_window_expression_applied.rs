extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

fn main() {
    use diesel::dsl::*;

    users::table.select(lag(users::name));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: Table` is not satisfied

    users::table.select(rank());
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: Table` is not satisfied
}
