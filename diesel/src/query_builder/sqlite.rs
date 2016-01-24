use backend::Sqlite;
use super::{QueryBuilder, BuildQueryResult, Context};
use types::HasSqlType;

#[doc(hidden)]
pub struct SqliteQueryBuilder {
    pub sql: String,
    context_stack: Vec<Context>,
}

impl SqliteQueryBuilder {
    pub fn new() -> Self {
        SqliteQueryBuilder {
            sql: String::new(),
            context_stack: Vec::new(),
        }
    }
}

impl QueryBuilder<Sqlite> for SqliteQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> BuildQueryResult {
        self.push_sql(identifier);
        Ok(())
    }

    fn push_bound_value<T>(&mut self, bind: Option<Vec<u8>>) where
        Sqlite: HasSqlType<T>,
    {
        match (self.context_stack.first(), bind) {
            (Some(&Context::Insert), None) => self.push_sql("NULL"),
            _ => self.push_sql("?"),
        }
    }

    fn push_context(&mut self, context: Context) {
        self.context_stack.push(context);
    }

    fn pop_context(&mut self) {
        self.context_stack.pop();
    }
}
