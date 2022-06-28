use crate::{NumberedParams, SqlValue, WhereBuilder};

const AND_OPERATOR: &str = "AND";

pub struct DeleteInner<'s> {
    where_builder: WhereBuilder<'s>,
    pub has_value: bool,
}

impl<'s> DeleteInner<'s> {
    pub fn new() -> Self {
        Self {
            where_builder: WhereBuilder::new(AND_OPERATOR),
            has_value: false,
        }
    }

    pub fn add_where_field(
        &'s mut self,
        numbered_params: &mut NumberedParams,
        field_name: &str,
        sql_value: SqlValue,
    ) {
        let sql_value = numbered_params.add_or_get(sql_value);
        self.where_builder.add(field_name, sql_value);
        self.has_value = true;
    }

    pub fn build(&'s mut self, table_name: &str, dest: &mut String) {
        dest.push_str("DELETE FROM ");
        dest.push_str(table_name);
        dest.push_str(" WHERE ");
        self.where_builder.build(dest);
    }
}
