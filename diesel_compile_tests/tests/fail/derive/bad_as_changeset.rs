use diesel::prelude::*;
//~^ ERROR: type mismatch resolving `<User as AsChangeset>::Changeset == _`

table! {
    users(id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

#[derive(AsChangeset)]
//~^ ERROR: type mismatch resolving `<User as AsChangeset>::Changeset == _`
//~| ERROR: the trait bound `&i32: AsExpression<Nullable<Text>>` is not satisfied
//~| ERROR: the trait bound `i32: AsExpression<diesel::sql_types::Text>` is not satisfied
//~| ERROR: the trait bound `i32: AsExpression<Nullable<Text>>` is not satisfied
//~| ERROR: the trait bound `&'update i32: AsExpression<diesel::sql_types::Text>` is not satisfied
//~| ERROR: the trait bound `i32: AsExpression<diesel::sql_types::Text>` is not satisfied
//~| ERROR: the trait bound `i32: AsExpression<Nullable<Text>>` is not satisfied
//~| ERROR: the type
struct User {
    id: String,
    name: i32,
    //~^ ERROR: the trait bound `i32: AsExpression<diesel::sql_types::Text>` is not satisfied
    hair_color: Option<i32>,
    //~^ ERROR: the trait bound `i32: AsExpression<Nullable<Text>>` is not satisfied
}

fn main() {}
