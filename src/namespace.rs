use crate::eval::Eval;
use crate::primitive::{Keyword, Operator};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    pub operators: HashMap<u8, Rc<Operator>>,
    pub keywords: HashMap<String, Rc<Keyword>>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut operators = HashMap::new();
        let mut keywords = HashMap::new();
        crate::load_builtin_ops!(operators,
            b'+' => crate::builtins::add,
            b'-' => crate::builtins::sub,
            b'*' => crate::builtins::mul,
            b'/' => crate::builtins::div,
            b'=' => crate::builtins::eq,
            b'>' => crate::builtins::ge,
        );
        crate::load_builtin_keywords!(keywords, String::from("if") => ());
        Self {
            operators,
            keywords,
        }
    }
}

impl Environment {
    pub fn operator_loaded(&self, id: &u8) -> bool {
        self.operators.contains_key(id)
    }
}
