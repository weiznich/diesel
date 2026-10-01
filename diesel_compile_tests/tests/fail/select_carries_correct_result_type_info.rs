extern crate diesel;

use diesel::*;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

fn main() {
    use self::users::dsl::*;

    let mut connection = PgConnection::establish("").unwrap();
    let select_id = users.select(id);
    let select_name = users.select(name);

    let ids = select_name.load::<i32>(&mut connection);
    //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, i32>` is not satisfied
    let names = select_id.load::<String>(&mut connection);
    //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, String>` is not satisfied
}
