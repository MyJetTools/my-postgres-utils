use crate::{NumberedParams, SqlValue, WhereBuilder};

const AND_OPERATOR: &str = "AND";
pub struct BulkDeleteBuilder<'s> {
    numbered_params: NumberedParams<'s>,
    where_builders: Vec<WhereBuilder<'s>>,
    current_where_bulder: Option<WhereBuilder<'s>>,
}

impl<'s> BulkDeleteBuilder<'s> {
    pub fn new() -> Self {
        Self {
            where_builders: Vec::new(),
            current_where_bulder: None,
            numbered_params: NumberedParams::new(),
        }
    }

    fn get_current_where_builder(&'s mut self) -> &'s mut WhereBuilder {
        match &mut self.current_where_bulder {
            Some(where_builder) => return where_builder,
            None => panic!("No Current Where builder"),
        }
    }

    fn add_new_line_int(&'s mut self, where_builder: WhereBuilder<'s>) {
        let old_value = std::mem::replace(&mut self.current_where_bulder, Some(where_builder));

        if let Some(old_line) = old_value {
            self.where_builders.push(old_line);
        }
    }

    pub fn add_new_line(&'s mut self) {
        self.add_new_line_int(WhereBuilder::new(AND_OPERATOR));
    }

    pub fn add_where_field(&'s mut self, field_name: &str, sql_value: SqlValue) {
        let sql_value = self.numbered_params.add_or_get(sql_value);
        self.get_current_where_builder().add(field_name, sql_value);
    }

    pub fn get_sql_line(&'s mut self, table_name: &str) -> String {
        let old_value = std::mem::replace(&mut self.current_where_bulder, None);

        if let Some(old_line) = old_value {
            self.where_builders.push(old_line);
        }

        let mut result = String::new();
        for where_builder in &self.where_builders {
            let where_clause = where_builder.build();
            result.push_str(&format!("DELETE FROM {table_name} WHERE {where_clause};"));
        }
        result
    }

    pub fn get_values_data(&'s mut self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        self.numbered_params.build_params()
    }
}
