use crate::{NumberedParams, SqlValue};

use super::DeleteInner;

pub struct BulkDeleteBuilder<'s> {
    numbered_params: NumberedParams<'s>,
    inners: Vec<DeleteInner>,
    current: DeleteInner,
}

impl<'s> BulkDeleteBuilder<'s> {
    pub fn new() -> Self {
        Self {
            current: DeleteInner::new(),
            numbered_params: NumberedParams::new(),
            inners: Vec::new(),
        }
    }

    pub fn add_new_line(&mut self) {
        let old_value = std::mem::replace(&mut self.current, DeleteInner::new());
        if old_value.has_value {
            self.inners.push(old_value);
        }
    }

    pub fn add_where_field(&mut self, field_name: &str, sql_value: SqlValue) {
        self.current
            .add_where_field(&mut self.numbered_params, field_name, sql_value);
    }

    pub fn build(&mut self, table_name: &str) -> String {
        let mut result = String::new();
        for inner in &mut self.inners {
            inner.build(table_name, &mut result);
            result.push(';');
        }

        if self.current.has_value {
            self.current.build(table_name, &mut result);
            result.push(';');
        }
        result
    }

    pub fn get_values_data(&'s mut self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
