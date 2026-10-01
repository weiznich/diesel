#[macro_use]
extern crate diesel;

#[derive(AsChangeset)]
//~^ ERROR: type annotations needed
struct User {
    //~^ ERROR: cannot find module or crate `users` in this scope
    id: i32,
    name: String,
}

#[derive(AsChangeset)]
//~^ ERROR: type annotations needed
#[diesel(table_name = users)]
//~^ ERROR: cannot find module or crate `users` in this scope
struct UserForm {
    id: i32,
    name: String,
}

fn main() {}
