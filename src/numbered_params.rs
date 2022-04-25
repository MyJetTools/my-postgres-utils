use std::collections::HashMap;

pub struct NumberedParams {
    params: HashMap<String, i32>,
    param_no: i32,
}

impl NumberedParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            param_no: 1,
        }
    }

    pub fn add_or_get(&mut self, name: &str) -> (i32, bool) {
        let mut exists = true;
        if !self.params.contains_key(name) {
            self.params.insert(name.to_string(), self.param_no);
            self.param_no += 1;
            exists = false;
        }

        return (self.params.get(name).unwrap().clone(), exists);
    }
}
