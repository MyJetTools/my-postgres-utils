use std::collections::HashMap;

use crate::SqlValue;

pub struct NumberedParams {
    params: HashMap<String, u32>,
    param_no: u32,
}

impl NumberedParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            param_no: 1,
        }
    }

    pub fn add_or_get(&mut self, sql_value: &SqlValue) -> Option<u32> {
        if let SqlValue::String(value) = sql_value {
            let mut exists = true;

            if !self.params.contains_key(value) {
                let result = self.param_no;
                self.params.insert(value.to_string(), result);
                self.param_no += 1;
                return Some(result);
            }

            return Some(*self.params.get(value).unwrap());
        }

        None
    }
}
