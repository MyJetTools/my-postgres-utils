pub struct BulkDeleteBuilder<'s> {
    values_data: Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
    where_clauses: Vec<String>,
}

impl<'s> BulkDeleteBuilder<'s> {
    pub fn new() -> Self {
        Self {
            values_data: Vec::new(),
            where_clauses: Vec::new(),
        }
    }

    pub fn add_where_clause(
        &mut self,
        where_clause: String,
        values_data: &[&'s (dyn tokio_postgres::types::ToSql + Sync)],
    ) {
        self.where_clauses.push(where_clause);
        self.values_data.extend_from_slice(values_data)
    }

    pub fn get_sql_line(&self, table_name: &str) -> String {
        let mut result = String::new();
        for where_clause in &self.where_clauses {
            result.push_str(&format!("DELETE FROM {table_name} WHERE {where_clause};"));
        }
        result
    }

    pub fn get_values_data(&'s self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        &self.values_data
    }
}
