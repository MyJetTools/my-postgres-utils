use crate::{sql_line_builder::SqlLineBuilder, NumberedParams, SqlValue};

pub struct InsertOrUpdateInner {
    insert_fields: SqlLineBuilder,
    insert_values: SqlLineBuilder,
    update_fields: SqlLineBuilder,
    pub has_value: bool,
}

impl InsertOrUpdateInner {
    pub fn new() -> Self {
        Self {
            insert_fields: SqlLineBuilder::new(','),
            insert_values: SqlLineBuilder::new(','),
            update_fields: SqlLineBuilder::new(','),
            has_value: false,
        }
    }

    pub fn add_field<'s>(
        &mut self,
        numbered_params: &mut NumberedParams<'s>,
        field_name: &str,
        sql_value: SqlValue,
        is_primary_key: bool,
    ) {
        let sql_value = numbered_params.add_or_get(sql_value);

        self.insert_fields.add(field_name);
        self.insert_values.add_sql_value(&sql_value);

        if !is_primary_key {
            self.update_fields.add_update(field_name, &sql_value);
        }
        self.has_value = true;
    }

    pub fn build(&self, dest: &mut String, table_name: &str, pk_name: &str) {
        dest.push_str("INSERT INTO  ");
        dest.push_str(table_name);
        dest.push_str(" (");
        dest.push_str(self.insert_fields.as_str());
        dest.push_str(") VALUES (");
        dest.push_str(self.insert_values.as_str());
        dest.push_str(") ON CONFLICT ON CONSTRAINT ");
        dest.push_str(pk_name);
        dest.push_str(" DO UPDATE SET ");
        dest.push_str(self.update_fields.as_str());
    }
}
