use sqlx_core::impl_statement_query;
use sqlx_core::sql_str::SqlStr;

#[derive(Clone)]
pub struct D1Statement {
    pub(crate) sql: SqlStr,
}

impl sqlx_core::statement::Statement for D1Statement {
    type Database = crate::D1;

    fn into_sql(self) -> SqlStr {
        self.sql
    }

    fn sql(&self) -> &SqlStr {
        &self.sql
    }

    fn parameters(
        &self,
    ) -> Option<
        sqlx_core::Either<&[<Self::Database as sqlx_core::database::Database>::TypeInfo], usize>,
    > {
        None
    }

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        &[]
    }

    impl_statement_query!(crate::arguments::D1Arguments);
}
