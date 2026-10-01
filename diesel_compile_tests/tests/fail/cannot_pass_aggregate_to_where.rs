extern crate diesel;

use diesel::dsl::count;
use diesel::*;

table! {
    users {
        id -> Integer,
    }
}

fn main() {
    use self::users::dsl::*;

    let source = users.filter(count(id).gt(3));
    //~^ ERROR: the trait bound `SelectStatement<FromClause<table>>: FilterDsl<_>` is not satisfied
}
