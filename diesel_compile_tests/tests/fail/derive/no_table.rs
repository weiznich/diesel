#![feature(diagnostic_on_unknown)]

#[macro_use]
extern crate diesel;

#[derive(AsChangeset)]
//~^ ERROR: type annotations needed
//~| ERROR: invalid table name `users` inferred
struct User {
    //~^ ERROR: invalid table name `users` inferred
    id: i32,
    name: String,
}

#[derive(AsChangeset)]
//~^ ERROR: type annotations needed
//~| ERROR: invalid table name `users` detected
#[diesel(table_name = users)]
//~^ ERROR: invalid table name `users` detected
struct UserForm {
    id: i32,
    name: String,
}

fn main() {}
