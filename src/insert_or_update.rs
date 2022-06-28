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
            insert_fields: SqlLineBuilder::new(" , ".to_string()),
            insert_values: SqlLineBuilder::new(" , ".to_string()),
            update_fields: SqlLineBuilder::new(" , ".to_string()),
            update_values: SqlLineBuilder::new(" , ".to_string()),

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
        format!(
            "INSERT INTO {table_name} {insert_fields} VALUES {insert_values} ON CONFLICT ON CONSTRAINT {pk_name} DO UPDATE SET ({udpate_fields}) = ({update_values})",
            insert_fields = self.insert_fields.as_str(),
            insert_values = self.insert_values.as_str(),
            udpate_fields = self.update_fields.as_str(),
            update_values = self.update_values.as_str(),
        )
    }
}
