use std::collections::HashMap;

use crate::SqlValue;

pub struct IndexedValuesGenerator {
    string_values: HashMap<String, u32>,
    index_no: u32,
}

impl IndexedValuesGenerator {
    pub fn new() -> Self {
        Self {
            string_values: HashMap::new(),
            index_no: 0,
        }
    }

    fn get_nexy_index(&mut self) -> u32 {
        self.index_no += 1;
        self.index_no
    }

    pub fn add_and_generate_index(&mut self, value: &SqlValue) -> Option<u32> {
        if let SqlValue::String(value) = value {
            if let Some(index) = self.string_values.get(value) {
                return Some(*index);
            }

            let next_index = self.get_nexy_index();
            self.string_values.insert(value.to_string(), next_index);
            return Some(next_index);
        }

        None
    }
}
