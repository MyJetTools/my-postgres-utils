use crate::{sql_line_builder::SqlLineBuilder, NumberedParams};

pub struct PosrgresInsertOrUpdateBuilder {
    insert_fields: SqlLineBuilder,
    insert_values: SqlLineBuilder,
    update_fields: SqlLineBuilder,
    update_values: SqlLineBuilder,

    numbered_params: NumberedParams,

    table_name: String,
    pk_name: String,
}

impl PosrgresInsertOrUpdateBuilder {
    pub fn new(table_name: String, pk_name: String) -> Self {
        Self {
            table_name,
            pk_name,
            insert_fields: SqlLineBuilder::new(" , ".to_string()),
            insert_values: SqlLineBuilder::new(" , ".to_string()),
            update_fields: SqlLineBuilder::new(" , ".to_string()),
            update_values: SqlLineBuilder::new(" , ".to_string()),

            numbered_params: NumberedParams::new(),
        }
    }

    pub fn append_insert_field<'s>(&'s mut self, field_name: &str) -> bool {
        let (param_no, exists) = self.numbered_params.add_or_get(field_name);
        self.insert_fields.add(field_name);
        self.insert_values.add(format!("${}", param_no).as_str());
        exists
    }

    pub fn append_insert_field_raw(&mut self, field_name: &str, value: &str) {
        self.insert_fields.add(field_name);
        self.insert_values.add(format!("'{}'", value).as_str());
    }

    pub fn append_update_field(&mut self, field_name: &str) -> bool {
        let (param_no, exists) = self.numbered_params.add_or_get(field_name);
        self.update_fields.add(field_name);
        self.update_values.add(format!("${}", param_no).as_str());
        exists
    }

    pub fn append_update_field_raw(&mut self, field_name: &str, value: &str) {
        self.update_fields.add(field_name);
        self.update_values.add(format!("'{}'", value).as_str());
    }

    pub fn get_sql_line(&self) -> String {
        format!(
            "INSERT INTO {table_name} {insert_fields} VALUES {insert_values} ON CONFLICT ON CONSTRAINT {pk_name} DO UPDATE SET ({udpate_fields}) = ({update_values})",
            table_name = self.table_name,
            pk_name = self.pk_name,
            insert_fields = self.insert_fields.as_str(),
            insert_values = self.insert_values.as_str(),
            udpate_fields = self.update_fields.as_str(),
            update_values = self.update_values.as_str(),
        )
    }
}
