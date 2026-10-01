extern crate diesel;

use diesel::dsl::{max, min};
use diesel::*;

table! {
    stuff (b) {
        b -> Bool,
    }
}

fn main() {
    let source = stuff::table.select(max(stuff::b));
    //~^ ERROR: the trait bound `Bool: SqlOrdAggregate` is not satisfied
    let source = stuff::table.select(min(stuff::b));
    //~^ ERROR: the trait bound `Bool: SqlOrdAggregate` is not satisfied
}
