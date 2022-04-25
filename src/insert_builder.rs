use crate::SqlLineBuilder;

pub struct PosrgresInsertBuilder<'s> {
    fields: SqlLineBuilder,
    values: SqlLineBuilder,
    value_no: u32,

    values_data: Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,
}

impl<'s> PosrgresInsertBuilder<'s> {
    pub fn new() -> Self {
        Self {
            fields: SqlLineBuilder::new(" , ".to_string()),
            values: SqlLineBuilder::new(" , ".to_string()),

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

    pub fn get_sql_line(&self, table_name: &str) -> String {
        format!(
            "INSERT INTO {table_name} ({fields}) VALUES ({values})",
            fields = self.fields.as_str(),
            values = self.values.as_str(),
        )
    }

    pub fn get_values_data(&'s self) -> &'s [&'s (dyn tokio_postgres::types::ToSql + Sync)] {
        &self.values_data
    }
}
