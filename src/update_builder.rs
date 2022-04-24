use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::SqlLineBuilder;

pub struct PosrgresUpdateBuilder<'s> {
    fields: SqlLineBuilder,
    values: SqlLineBuilder,
    value_no: u32,
    where_clause: SqlLineBuilder,
    values_data: Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
}

impl<'s> PosrgresUpdateBuilder<'s> {
    pub fn new() -> Self {
        Self {
            fields: SqlLineBuilder::new(" , ".to_string()),
            values: SqlLineBuilder::new(" , ".to_string()),
            where_clause: SqlLineBuilder::new(" AND ".to_string()),
            values_data: Vec::new(),
            value_no: 1,
        }
    }

    pub fn append_field(
        &mut self,
        field_name: &str,
        value: &'s (dyn tokio_postgres::types::ToSql + Sync),
    ) {
        self.fields.add(field_name);
        self.values.add(format!("${}", self.value_no).as_str());
        self.values_data.push(value);
        self.value_no += 1;
    }

    pub fn append_field_raw(&mut self, field_name: &str, value: &str) {
        self.fields.add(field_name);
        self.values.add(format!("'{}'", value).as_str());
    }

    pub fn append_where(
        &mut self,
        field_name: &str,
        value: &'s (dyn tokio_postgres::types::ToSql + Sync),
    ) {
        self.where_clause
            .add(format!("{field_name}=${}", self.value_no).as_str());
        self.values_data.push(value);

        self.value_no += 1;
    }

    pub fn append_where_raw(&mut self, field_name: &str, value: &str) {
        self.where_clause
            .add(format!("{field_name}='{}'", value).as_str());
    }

    pub fn get_sql_line(&self, table_name: &str) -> String {
        format!(
            "UPDATE {table_name} SET ({fields})=({values}) WHERE {where_clause}",
            fields = self.fields.as_str(),
            values = self.values.as_str(),
            where_clause = self.where_clause.as_str()
        )
    }

    pub fn get_values_data(&'s self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        &self.values_data
    }
}
