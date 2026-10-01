extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

fn main() {
    use diesel::dsl::sum;

    let _ = users::table.filter(users::name);
    //~^ ERROR: the trait bound `SelectStatement<_>: FilterDsl<name>` is not satisfied
    let _ = users::table.filter(sum(users::id).eq(1));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: FilterDsl<_>` is not satisfied
}
