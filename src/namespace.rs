use crate::prim::{Operator, Value};
use std::rc::Rc;

pub struct OperatorTable {
    pub(crate) operators: std::collections::HashMap<u8, Operator>,
}

impl Default for OperatorTable {
    fn default() -> Self {
        Self::init()
    }
}

impl OperatorTable {
    pub fn init() -> Self {
        let mut table = Self {
            operators: std::collections::HashMap::new(),
        };

        super::builtins::load_operators(&mut table);

        log::debug!(
            "Preloaded builtins: {:?}",
            table.operators.keys().collect::<Vec<_>>()
        );

        table
    }

    pub(crate) fn add_operator(
        table: &mut std::collections::HashMap<u8, Operator>,
        id: u8,
        f: fn(&[Rc<Value>]) -> Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if table.contains_key(&id) {
            Err("Operator already exists".into())
        } else {
            let op = Operator::new(id.clone(), f);
            table.insert(id, op);
            Ok(())
        }
    }

    pub fn get(&self, id: &u8) -> Option<&Operator> {
        self.operators.get(id)
    }
}
