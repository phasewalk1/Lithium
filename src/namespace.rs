use crate::prim::{
    Atom,
    Operator,
    Value,
};
use std::rc::Rc;

pub struct OperatorTable {
    operators: std::collections::HashMap<u8, Operator>,
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

        OperatorTable::add_operator(&mut table.operators, b'+', |args| {
            let mut sum = 0;
            for arg in args {
                if let Value::Atom(Atom(n)) = arg.as_ref() {
                    sum += n;
                } else {
                    return Value::Nil;
                }
            }
            Value::Atom(Atom(sum))
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'*', |args| {
            let mut product = 1;
            for arg in args {
                if let Value::Atom(Atom(n)) = arg.as_ref() {
                    product *= n;
                } else {
                    return Value::Nil;
                }
            }
            Value::Atom(Atom(product))
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'-', |args| {
            if let Value::Atom(Atom(n)) = args[0].as_ref() {
                if let Value::Atom(Atom(m)) = args[1].as_ref() {
                    Value::Atom(Atom(n - m))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'/', |args| {
            if let Value::Atom(Atom(n)) = args[0].as_ref() {
                if let Value::Atom(Atom(m)) = args[1].as_ref() {
                    Value::Atom(Atom(n / m))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'=', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a == b { Value::T } else { Value::Nil }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'<', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a < b { Value::T } else { Value::Nil }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }).unwrap();

        OperatorTable::add_operator(&mut table.operators, b'>', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a > b { Value::T } else { Value::Nil }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }).unwrap();

        log::debug!("Preloaded builtins: {:?}", table.operators.keys().collect::<Vec<_>>());

        table
    }

    fn add_operator(
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
