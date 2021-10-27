use crate::backend::{sql_dialect, Backend};
use crate::expression::SqlLiteral;
use crate::query_builder::*;
use crate::query_source::Column;
use crate::result::QueryResult;

#[doc(hidden)]
pub trait OnConflictTarget<Table> {}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoConflictTarget;

impl<DB> QueryFragment<DB> for NoConflictTarget
where
    DB: Backend,
    DB::OnConflictClause: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
{
    fn walk_ast<'a, 'b>(&'a self, _: AstPass<'_, 'b, DB>) -> QueryResult<()>
    where
        'a: 'b,
    {
        Ok(())
    }
}

impl<Table> OnConflictTarget<Table> for NoConflictTarget {}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ConflictTarget<T>(pub T);

impl<DB, T> QueryFragment<DB> for ConflictTarget<T>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::OnConflictClause>,
{
    fn walk_ast<'a, 'b>(&'a self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()>
    where
        'a: 'b,
    {
        <Self as QueryFragment<DB, DB::OnConflictClause>>::walk_ast(self, pass)
    }
}

impl<DB, T, SP> QueryFragment<DB, SP> for ConflictTarget<T>
where
    DB: Backend<OnConflictClause = SP>,
    SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    T: Column,
{
    fn walk_ast<'a, 'b>(&'a self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()>
    where
        'a: 'b,
    {
        out.push_sql(" (");
        out.push_identifier(T::NAME)?;
        out.push_sql(")");
        Ok(())
    }
}

impl<T> OnConflictTarget<T::Table> for ConflictTarget<T> where T: Column {}

impl<DB, ST, SP> QueryFragment<DB, SP> for ConflictTarget<SqlLiteral<ST>>
where
    DB: Backend<OnConflictClause = SP>,
    SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    SqlLiteral<ST>: QueryFragment<DB>,
{
    fn walk_ast<'a, 'b>(&'a self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()>
    where
        'a: 'b,
    {
        out.push_sql(" ");
        self.0.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<Tab, ST> OnConflictTarget<Tab> for ConflictTarget<SqlLiteral<ST>> {}

impl<DB, T, SP> QueryFragment<DB, SP> for ConflictTarget<(T,)>
where
    DB: Backend<OnConflictClause = SP>,
    SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    T: Column,
{
    fn walk_ast<'a, 'b>(&'a self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()>
    where
        'a: 'b,
    {
        out.push_sql(" (");
        out.push_identifier(T::NAME)?;
        out.push_sql(")");
        Ok(())
    }
}

impl<T> OnConflictTarget<T::Table> for ConflictTarget<(T,)> where T: Column {}

macro_rules! on_conflict_tuples {
    ($(
        $Tuple:tt {
            $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)*
        }
    )+) => {
        $(
            impl<_DB, _T, _SP, $($T),*> QueryFragment<_DB, _SP> for ConflictTarget<(_T, $($T),*)> where
                _DB: Backend<OnConflictClause = _SP>,
                _SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
                _T: Column,
                $($T: Column<Table=_T::Table>,)*
            {
                fn walk_ast<'a, 'b>(&'a self, mut out: AstPass<'_, 'b, _DB>) -> QueryResult<()>
                where 'a: 'b
                {
                    out.push_sql(" (");
                    out.push_identifier(_T::NAME)?;
                    $(
                        out.push_sql(", ");
                        out.push_identifier($T::NAME)?;
                    )*
                    out.push_sql(")");
                    Ok(())
                }
            }

            impl<_T, $($T),*> OnConflictTarget<_T::Table> for ConflictTarget<(_T, $($T),*)> where
                _T: Column,
                $($T: Column<Table=_T::Table>,)*
            {
            }
        )*
    }
}

__diesel_for_each_tuple!(on_conflict_tuples);
