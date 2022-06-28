use crate::SqlValue;

pub struct SqlLineBuilder {
    result: String,
    separator: String,
}

impl SqlLineBuilder {
    pub fn new(separator: String) -> Self {
        Self {
            result: "".to_string(),
            separator: separator,
        }
    }

    pub fn add(&mut self, value: &str) {
        if self.result.len() > 0 {
            self.result.push_str(self.separator.as_str());
        }

        self.result.push_str(value);
    }

    pub fn add_sql_value(&mut self, sql_value: &SqlValue) {
        if self.result.len() > 0 {
            self.result.push_str(self.separator.as_str());
        }

        self.result
            .push_str(sql_value.as_sql_value_to_injext().as_str());
    }

    pub fn as_str(&self) -> &str {
        self.result.as_str()
    }
}
