use super::{BatchChangeset, BatchUpdateQueryFragmentHelper};
use crate::backend::Backend;
use crate::expression::grouped::Grouped;
use crate::expression::operators::Eq;
use crate::expression::AppearsOnTable;
use crate::query_builder::*;
use crate::query_source::{Column, QuerySource};
use crate::result::QueryResult;
use crate::sql_types::HasSqlType;

/// Types which can be passed to
/// [`update.set`](UpdateStatement::set()).
///
/// This trait can be [derived](derive@AsChangeset)
pub trait AsChangeset {
    /// The table which `Self::Changeset` will be updating
    type Target: QuerySource;

    /// The update statement this type represents
    type Changeset;

    /// Convert `self` into the actual update statement being executed
    // This method is part of our public API
    // we won't change it to just appease clippy
    #[allow(clippy::wrong_self_convention)]
    fn as_changeset(self) -> Self::Changeset;
}

pub trait AsChangesetImpl {
    type Impl;
}

#[derive(Debug, Clone, Copy)]
pub struct SingleUpdate;
#[derive(Debug, Clone, Copy)]
pub struct BatchUpdate;

#[doc(inline)]
pub use diesel_derives::AsChangeset;

impl<T: AsChangeset> AsChangeset for Option<T> {
    type Target = T::Target;
    type Changeset = Option<T::Changeset>;

    fn as_changeset(self) -> Self::Changeset {
        self.map(AsChangeset::as_changeset)
    }
}

impl<T> AsChangesetImpl for Option<T>
where
    T: AsChangesetImpl<Impl = SingleUpdate>,
{
    type Impl = SingleUpdate;
}

impl<Left, Right> AsChangeset for Eq<Left, Right>
where
    Left: Column,
    Right: AppearsOnTable<Left::Table>,
{
    type Target = Left::Table;
    type Changeset = Assign<Left, Right>;

    fn as_changeset(self) -> Self::Changeset {
        Assign {
            column: self.left,
            expr: self.right,
        }
    }
}

impl<Left, Right> AsChangeset for Grouped<Eq<Left, Right>>
where
    Eq<Left, Right>: AsChangeset,
{
    type Target = <Eq<Left, Right> as AsChangeset>::Target;

    type Changeset = <Eq<Left, Right> as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        self.0.as_changeset()
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Assign<Col, Expr> {
    column: Col,
    expr: Expr,
}

impl<T, U, DB> QueryFragment<DB> for Assign<T, U>
where
    DB: Backend,
    T: Column,
    U: QueryFragment<DB>,
{
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_identifier(T::NAME)?;
        out.push_sql(" = ");
        QueryFragment::walk_ast(&self.expr, out)
    }
}

impl<T, U> AsChangesetImpl for Assign<T, U> {
    type Impl = SingleUpdate;
}

impl<DB, C, U> BatchUpdateQueryFragmentHelper<DB> for Assign<C, U>
where
    DB: Backend + HasSqlType<C::SqlType>,
    C: Column,
    U: QueryFragment<DB>,
{
    fn walk_values(&self, out: AstPass<DB>) -> QueryResult<()> {
        self.expr.walk_ast(out)
    }

    fn walk_column_list(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_identifier(C::NAME)
    }

    fn walk_assign_list(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        self.column.walk_ast(out.reborrow())?;
        out.push_sql(" = ");
        out.push_identifier("r")?;
        out.push_sql(".");
        out.push_identifier(C::NAME)?;
        Ok(())
    }
}

impl<T> AsChangeset for Vec<T>
where
    T: AsChangeset,
{
    type Target = T::Target;

    type Changeset = BatchChangeset<Vec<T::Changeset>>;

    fn as_changeset(self) -> Self::Changeset {
        BatchChangeset::new(self.into_iter().map(AsChangeset::as_changeset).collect())
    }
}

impl<T> AsChangesetImpl for BatchChangeset<T> {
    type Impl = BatchUpdate;
}
