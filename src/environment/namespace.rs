use crate::primitive::{Keyword, Operator};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    pub operators: HashMap<u8, Rc<Operator>>,
    pub keywords: HashMap<String, Rc<Keyword>>,
}

impl Environment {
    pub fn operator_loaded(&self, id: &u8) -> bool {
        self.operators.contains_key(id)
    }
}
