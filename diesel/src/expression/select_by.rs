use crate::backend::Backend;
use crate::expression::{
    AppearsOnTable, Expression, QueryMetadata, Selectable, SelectableExpression, ValidGrouping,
};
use crate::query_builder::*;
use crate::result::QueryResult;

#[derive(Debug, Default)]
pub struct SelectBy<T>(std::marker::PhantomData<T>);

impl<T> Clone for SelectBy<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T> Copy for SelectBy<T> {}

impl<T, ST> QueryId for SelectBy<T>
where
    T: Selectable<Expression = ST>,
    ST: QueryId + Expression,
{
    type QueryId = ST::QueryId;

    const HAS_STATIC_QUERY_ID: bool = ST::HAS_STATIC_QUERY_ID;
}

impl<T> SelectBy<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T, ST> Expression for SelectBy<T>
where
    T: Selectable<Expression = ST>,
    ST: QueryId + Expression,
{
    type SqlType = ST::SqlType;
}

impl<T, GB, ST> ValidGrouping<GB> for SelectBy<T>
where
    T: Selectable<Expression = ST>,
    ST: Expression + ValidGrouping<GB>,
{
    type IsAggregate = ST::IsAggregate;
}

impl<ST, DB> QueryMetadata<SelectBy<ST>> for DB
where
    DB: Backend,
{
    fn row_metadata(_: &Self::MetadataLookup, out: &mut Vec<Option<Self::TypeMetadata>>) {
        out.push(None)
    }
}

impl<T, DB> QueryFragment<DB> for SelectBy<T>
where
    T: Selectable,
    T::Expression: QueryFragment<DB>,
    DB: Backend,
{
    fn walk_ast(&self, out: AstPass<DB>) -> QueryResult<()> {
        T::new_expression().walk_ast(out)
    }
}

impl<T, QS> SelectableExpression<QS> for SelectBy<T>
where
    T: Selectable,
    T::Expression: SelectableExpression<QS>,
    Self: AppearsOnTable<QS>,
{
}

impl<T, QS> AppearsOnTable<QS> for SelectBy<T>
where
    T: Selectable,
    T::Expression: AppearsOnTable<QS>,
    Self: Expression,
{
}
