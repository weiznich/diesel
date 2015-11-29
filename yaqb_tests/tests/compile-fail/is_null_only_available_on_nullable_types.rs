#[macro_use]
extern crate yaqb;

use yaqb::*;

table! {
    users {
        id -> Serial,
    }
}

fn main() {
    let _ = users::id.is_null();
    //~^ ERROR error: no method named `is_null` found
}
