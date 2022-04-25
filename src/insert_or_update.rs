use crate::{sql_line_builder::SqlLineBuilder, NumberedParams};

pub struct PosrgresInsertOrUpdateBuilder<'s> {
    insert_fields: SqlLineBuilder,
    insert_values: SqlLineBuilder,
    update_fields: SqlLineBuilder,
    update_values: SqlLineBuilder,

    numbered_params: NumberedParams,

    values_data: Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
}

impl<'s> PosrgresInsertOrUpdateBuilder<'s> {
    pub fn new() -> Self {
        Self {
            insert_fields: SqlLineBuilder::new(" , ".to_string()),
            insert_values: SqlLineBuilder::new(" , ".to_string()),
            update_fields: SqlLineBuilder::new(" , ".to_string()),
            update_values: SqlLineBuilder::new(" , ".to_string()),
            values_data: Vec::new(),
            numbered_params: NumberedParams::new(),
        }
    }

    pub fn append_insert_field(
        &mut self,
        field_name: &str,
        value: &'s (dyn tokio_postgres::types::ToSql + Sync),
    ) {
        let (param_no, exists) = self.numbered_params.add_or_get(field_name);
        self.insert_fields.add(field_name);
        self.insert_values.add(format!("${}", param_no).as_str());
        self.values_data.push(value);

        if !exists {
            self.values_data.push(value);
        }
    }

    pub fn append_insert_field_raw(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        self.insert_values.add(format!("'{}'", value).as_str());
    }

    pub fn append_update_field(
        &mut self,
        field_name: &str,
        value: &'s (dyn tokio_postgres::types::ToSql + Sync),
    ) {
        let (param_no, exists) = self.numbered_params.add_or_get(field_name);
        self.update_fields.add(field_name);
        self.update_values.add(format!("${}", param_no).as_str());

        if !exists {
            self.values_data.push(value);
        }
    }

    pub fn append_update_field_raw(&mut self, field_name: &str, value: &str) {
        self.update_fields.add(field_name);
        self.update_values.add(format!("'{}'", value).as_str());
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

    pub fn get_values_data(&'s self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        &self.values_data
    }
}
