use super::{AstPass, IntoBinds, QueryFragment};
use crate::backend::Backend;
use crate::result::QueryResult;

struct OwnedQuery<T>
// where
//     DB: Backend,
// for<'a, 'b> <DB as Backend>::BindCollector<'a>: IntoBinds<'b, DB>,
{
    sql: String,
    safe_to_cache_prepared: bool,
    binds: Vec<T>,
    // binds: Vec<<<DB as Backend>::BindCollector<'param> as IntoBinds<'param, DB>>::OwnedBuffer>,
}

impl<DB, T> QueryFragment<DB> for OwnedQuery<T>
where
    DB: Backend,
    for<'a> <DB as Backend>::BindCollector<'a>: IntoBinds<'a, DB>,
    // for<'a> DB: Backend<BindCollector<'a> as IntoBinds<'a, DB>::OwnedBuffer = T>,
    // binds: Vec<<<DB as Backend>::BindCollector<'param> as IntoBinds<'param, DB>>::OwnedBuffer>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        if !self.safe_to_cache_prepared {
            pass.unsafe_to_cache_prepared();
        }
        pass.push_sql(&self.sql);
        // pass.push_bind_collector_data(self);
        // for bind in self.binds {
        //     pass.push_bind_param_value_only(&bind);
        // }
        Ok(())
    }
}
