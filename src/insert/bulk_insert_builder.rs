use crate::sql_line_builder::SqlLineBuilder;

pub struct BulkInsertBuilder<'s> {
    fields: SqlLineBuilder,
    values: Vec<SqlLineBuilder>,
    value_no: u32,
    current_value: Option<SqlLineBuilder>,
    values_data: Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
}

impl<'s> BulkInsertBuilder<'s> {
    pub fn new() -> Self {
        Self {
            fields: SqlLineBuilder::new(",".to_string()),
            values: Vec::new(),
            current_value: None,
            values_data: Vec::new(),
            value_no: 0,
        }
    }

    pub fn append_field(&mut self, field_name: &str) {
        self.fields.add(field_name)
    }

    pub fn start_new_value_line(&mut self) {
        let mut new_line = Some(SqlLineBuilder::new(",".to_string()));

        std::mem::swap(&mut self.current_value, &mut new_line);

        if let Some(new_line) = new_line {
            self.values.push(new_line);
        }
    }

    pub fn append_value(&mut self, value: &'s (dyn tokio_postgres::types::ToSql + Sync)) {
        if let Some(current_value) = &mut self.current_value {
            current_value.add(format!("${}", self.value_no).as_str());
            self.values_data.push(value);
            self.value_no += 1;
        } else {
            panic!("Current value is not set");
        }
    }

    pub fn append_value_raw(&mut self, value: &str) {
        if let Some(current_value) = &mut self.current_value {
            current_value.add(format!("'{}'", value).as_str());
        } else {
            panic!("Current value is not set");
        }
    }

    pub fn get_sql_line(&mut self, table_name: &str) -> String {
        let mut new_line = None;

        std::mem::swap(&mut self.current_value, &mut new_line);

        let mut result = format!(
            "INSERT INTO {table_name} ({fields}) VALUES ",
            fields = self.fields.as_str(),
        );

        let no = 0;
        for value in &self.values {
            if no > 0 {
                result.push(',');
            }
            result.push('(');
            result.push_str(value.as_str());
            result.push(')');
        }

        result
    }

    pub fn get_values_data(&'s self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        &self.values_data
    }
}
