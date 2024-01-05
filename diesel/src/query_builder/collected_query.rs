use super::{AstPass, BindCollector, MovableBindCollector, QueryFragment, QueryId};
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::result::QueryResult;

pub struct CollectedQuery<T>
// where
//     DB: Backend,
// for<'a, 'b> <DB as Backend>::BindCollector<'a>: IntoBinds<'b, DB>,
{
    sql: String,
    safe_to_cache_prepared: bool,
    movable_bind_collector: T,
    // binds: Vec<<<DB as Backend>::BindCollector<'param> as IntoBinds<'param, DB>>::OwnedBuffer>,
}

impl<T> CollectedQuery<T> {
    pub fn new(sql: String, safe_to_cache_prepared: bool, movable_bind_collector: T) -> Self {
        Self {
            sql,
            safe_to_cache_prepared,
            movable_bind_collector,
        }
    }
}

impl<DB, T> QueryFragment<DB> for CollectedQuery<T>
where
    DB: Backend + DieselReserveSpecialization,
    for<'a> <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB, MovableData = T>,
    // for<'a> T: MovableBindCollector<'a, DB>,
    // for<'a> DB: Backend<BindCollector<'a> as IntoBinds<'a, DB>::OwnedBuffer = T>,
    // binds: Vec<<<DB as Backend>::BindCollector<'param> as IntoBinds<'param, DB>>::OwnedBuffer>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        if !self.safe_to_cache_prepared {
            pass.unsafe_to_cache_prepared();
        }
        pass.push_sql(&self.sql);
        pass.push_bind_collector_data::<T>(&self.movable_bind_collector)
    }
}

impl<T> QueryId for CollectedQuery<T> {
    type QueryId = ();

    const HAS_STATIC_QUERY_ID: bool = false;
}
