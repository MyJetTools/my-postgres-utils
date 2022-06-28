use crate::{sql_line_builder::SqlLineBuilder, NumberedParams, SqlValue};

pub struct PosrgresInsertOrUpdateBuilder<'s> {
    insert_fields: SqlLineBuilder,
    insert_values: SqlLineBuilder,
    update_fields: SqlLineBuilder,
    update_values: SqlLineBuilder,
    numbered_params: NumberedParams<'s>,
}

impl<'s> PosrgresInsertOrUpdateBuilder<'s> {
    pub fn new() -> Self {
        Self {
            insert_fields: SqlLineBuilder::new(','),
            insert_values: SqlLineBuilder::new(','),
            update_fields: SqlLineBuilder::new(','),
            update_values: SqlLineBuilder::new(','),

            numbered_params: NumberedParams::new(),
        }
    }

    pub fn append_insert_field(&mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);

        self.insert_fields.add(field_name);
        self.insert_values.add_sql_value(&sql_value);
    }

    pub fn append_update_field(&mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);

        self.update_fields.add(field_name);
        self.update_values.add_sql_value(&sql_value);
    }

    pub fn get_sql_line(&self, table_name: &str, pk_name: &str) -> String {
        let mut result = String::new();

        result.push_str("INSERT INTO  ");
        result.push_str(table_name);
        result.push(' ');
        result.push_str(self.insert_fields.as_str());
        result.push_str(" VALUES ");
        result.push_str(self.insert_values.as_str());
        result.push_str(" ON CONFLICT ON CONSTRAINT ");
        result.push_str(pk_name);
        result.push_str(" DO UPDATE SET (");
        result.push_str(self.update_fields.as_str());
        result.push_str(") = (");
        result.push_str(self.update_values.as_str());
        result.push_str(")");

        result
    }
}
