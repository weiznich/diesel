#[macro_use]
extern crate diesel;

use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

table! {
    posts {
        id -> Integer,
        author -> Integer,
        title -> Text,
    }
}

table! {
    pets {
        id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(users, posts, pets);
joinable!(posts -> users (author));

pub fn check(conn: &mut PgConnection) {
    let user_alias = alias!(users as users2);
    let post_alias = alias!(posts as posts2);

    // wrong fields

    user_alias.field(posts::id);
    //~^ ERROR: type mismatch resolving `<id as QueryRelationField>::QueryRelation == table`

    // joining the same alias twice

    users::table
        .inner_join(post_alias)
        .inner_join(post_alias)
        //~^ ERROR: the trait bound `SelectStatement<_>: InternalJoinDsl<Alias<posts2>, Inner, _>` is not satisfied
        .select(users::id)
        .load::<i32>(conn)
        .unwrap();

    // Selecting the raw field on the aliased table
    user_alias.select(users::id).load::<i32>(conn).unwrap();
    //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, i32>` is not satisfied
    //~| ERROR: the trait bound `Alias<users2>: SelectDsl<users::columns::id>` is not satisfied

    let user2_alias = alias!(users as user3);

    // don't allow joins to not joinable tables
    pets::table
        .inner_join(user_alias)
        //~^ ERROR: the trait bound `table: JoinWithImplicitOnClause<Alias<users2>, Inner>` is not satisfied
        .select(pets::id)
        .load::<i32>(conn)
        .unwrap();

    // Check how error message looks when aliases to the same table are declared separately
    let post_alias_2 = alias!(posts as posts3);
    let posts = post_alias
        .inner_join(
            post_alias_2.on(post_alias
                .field(posts::author)
                .eq(post_alias_2.field(posts::author))),
            //~^^^ ERROR: the trait bound `Alias<posts2>: InternalJoinDsl<Alias<posts3>, Inner, _>` is not satisfied
        )
        .select((post_alias.field(posts::id), post_alias_2.field(posts::id)))
        //~^ ERROR: the trait bound `SelectStatement<FromClause<_>>: SelectDsl<_>` is not satisfied
        .load::<(i32, i32)>(conn)
        //~^ ERROR: the trait bound `SelectStatement<_, _>: LoadQuery<'_, _, _>` is not satisfied
        .unwrap();
}

fn main() {}
