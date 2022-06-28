use crate::{sql_line_builder::SqlLineBuilder, NumberedParams, SqlValue};

pub struct BulkInsertBuilder<'s> {
    fields: SqlLineBuilder,
    values: Vec<SqlLineBuilder>,
    current_value: SqlLineBuilder,
    numbered_params: NumberedParams<'s>,
}

impl<'s> BulkInsertBuilder<'s> {
    pub fn new() -> Self {
        Self {
            fields: SqlLineBuilder::new(','),
            values: Vec::new(),
            current_value: SqlLineBuilder::new(','),
            numbered_params: NumberedParams::new(),
        }
    }

    pub fn append_field(&mut self, field_name: &str) {
        self.fields.add(field_name)
    }

    pub fn start_new_value_line(&mut self) {
        if !self.current_value.has_value() {
            return;
        }

        let old_value = std::mem::replace(&mut self.current_value, SqlLineBuilder::new(','));
        self.values.push(old_value);
    }

    pub fn append_value(&mut self, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);
        self.current_value.add_sql_value(&sql_value);
    }

    pub fn build(&mut self, table_name: &str) -> String {
        let mut result = format!(
            "INSERT INTO {table_name} ({fields}) VALUES ",
            fields = self.fields.as_str(),
        );

        let mut no = 0;
        for value in &self.values {
            if no > 0 {
                result.push(',');
            }
            result.push('(');
            result.push_str(value.as_str());
            result.push(')');
            no += 1;
        }

        if self.current_value.has_value() {
            if no > 0 {
                result.push(',');
            }
            result.push('(');
            result.push_str(self.current_value.as_str());
            result.push(')');
        }
        result
    }

    pub fn get_values_data(&mut self) -> &'s [&(dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
