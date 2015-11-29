use expression::Expression;
use expression::predicates::*;
use types::{NativeSqlType, Nullable};

pub trait NullableExpressionMethods<ST>: Expression + Sized {
    fn is_null(self) -> IsNull<Self> {
       IsNull::new(self)
    }

    fn is_not_null(self) -> IsNotNull<Self> {
       IsNotNull::new(self)
    }
}

impl<ST, T> NullableExpressionMethods<ST> for T where
    ST: NativeSqlType,
    T: Expression<SqlType=Nullable<ST>>
{
}
