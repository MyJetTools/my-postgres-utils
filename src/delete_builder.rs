use crate::{NumberedParams, SqlValue, WhereBuilder};

const AND_OPERATOR: &str = "AND";
pub struct DeleteBuilder<'s> {
    numbered_params: &'s mut NumberedParams<'s>,
    where_builder: WhereBuilder<'s>,
}

impl<'s> DeleteBuilder<'s> {
    pub fn new(numbered_params: &'s mut NumberedParams<'s>) -> Self {
        Self {
            where_builder: WhereBuilder::new(AND_OPERATOR),
            numbered_params,
        }
    }

    pub fn add_where_field(&'s mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);
        self.where_builder.add(field_name, sql_value);
    }

    pub fn get_sql_line(&'s mut self, table_name: &str) -> String {
        let mut result = String::new();

        let where_clause = self.where_builder.build();
        result.push_str(&format!("DELETE FROM {table_name} WHERE {where_clause};"));

        result
    }

    pub fn get_values_data(&'s mut self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
