use crate::{NumberedParams, SqlLineBuilder, SqlValue, WhereBuilder};

pub struct PosrgresUpdateBuilder<'s> {
    fields: SqlLineBuilder,
    values: SqlLineBuilder,
    where_clause: WhereBuilder,
    numbered_params: NumberedParams<'s>,
}

impl<'s> PosrgresUpdateBuilder<'s> {
    pub fn new() -> Self {
        Self {
            fields: SqlLineBuilder::new(','),
            values: SqlLineBuilder::new(','),
            where_clause: WhereBuilder::new("AND"),
            numbered_params: NumberedParams::new(),
        }
    }

    pub fn append_field(&mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);
        self.fields.add(field_name);
        self.values.add_sql_value(&sql_value)
    }

    pub fn append_where(&mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);
        self.where_clause.add(field_name, sql_value)
    }

    pub fn build(&self, table_name: &str) -> String {
        let mut result = String::new();
        result.push_str("UPDATE ");
        result.push_str(table_name);
        result.push_str(" SET (");
        result.push_str(self.fields.as_str());
        result.push_str(")=(");
        result.push_str(self.values.as_str());
        result.push_str(") WHERE ");

        self.where_clause.build(&mut result);

        result
    }
    pub fn get_values_data(&mut self) -> &'s [&(dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
