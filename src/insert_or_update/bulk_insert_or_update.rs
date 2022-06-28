use crate::{NumberedParams, SqlValue};

use super::InsertOrUpdateInner;

pub struct BulkInsertOrUpdateBuilder<'s> {
    lines: Vec<InsertOrUpdateInner>,
    numbered_params: NumberedParams<'s>,
    current: InsertOrUpdateInner,
}

impl<'s> BulkInsertOrUpdateBuilder<'s> {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            numbered_params: NumberedParams::new(),
            current: InsertOrUpdateInner::new(),
        }
    }

    pub fn add_new_line(&mut self) {
        if !self.current.has_value {
            return;
        }
        let old_value = std::mem::replace(&mut self.current, InsertOrUpdateInner::new());
        if old_value.has_value {
            self.lines.push(old_value);
        }
    }

    pub fn add_field(&mut self, field_name: &str, sql_value: SqlValue, is_primary_key: bool) {
        self.current.add_field(
            &mut self.numbered_params,
            field_name,
            sql_value,
            is_primary_key,
        )
    }

    pub fn build(&self, table_name: &str, pk_name: &str) -> String {
        let mut result = String::new();
        result.push_str("BEGIN;");

        for line in &self.lines {
            line.build(&mut result, table_name, pk_name);
            result.push(';');
        }

        if self.current.has_value {
            self.current.build(&mut result, table_name, pk_name);
            result.push(';');
        }

        result.push_str("COMMIT;");

        result
    }

    pub fn get_values_data(&mut self) -> &'s [&(dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
