use crate::SqlValue;

pub struct WhereBuilder {
    operator: String,
    values: Vec<(String, SqlValue)>,
}

impl WhereBuilder {
    pub fn new(operator: String) -> Self {
        Self {
            operator,
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, field_name: &str, value: SqlValue) {
        self.values.push((field_name.to_string(), value));
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for (field_name, value) in &self.values {
            if result.len() > 0 {
                result.push(' ');
                result.push_str(&self.operator);
                result.push(' ');
            }

            result.push_str(field_name.as_str());
            result.push('=');
            result.push_str(value.as_sql_value_to_injext().as_str());
        }
        result
    }
}
