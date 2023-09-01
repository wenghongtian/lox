use std::collections::HashMap;

use crate::{
    error::LoxError,
    hashmap,
    token::{Object, Token},
};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            Err(LoxError::new_runtime(format!(
                "Undefined variable '{}'.",
                name.lexeme
            )))
        }
    }

    pub fn new() -> Self {
        Self { values: hashmap!() }
    }
}
