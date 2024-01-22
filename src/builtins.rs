use crate::namespace::OperatorTable;
use crate::prim::{Atom, Value};

pub fn load_operators(table: &mut OperatorTable) {
    crate::ops_builtin!(
        table,
        (b'+', |args| {
            assert_eq!(args.len(), 2);
            let mut sum = 0;
            for arg in args {
                if let Value::Atom(Atom(n)) = arg.as_ref() {
                    sum += n;
                } else {
                    return Value::Nil;
                }
            }
            Value::Atom(Atom(sum))
        }),
        (b'-', |args| {
            assert_eq!(args.len(), 2);
            let mut sum = 0;
            for arg in args {
                if let Value::Atom(Atom(n)) = arg.as_ref() {
                    sum -= n;
                } else {
                    return Value::Nil;
                }
            }
            Value::Atom(Atom(sum))
        }),
        (b'*', |args| {
            assert_eq!(args.len(), 2);
            let mut product = 1;
            for arg in args {
                if let Value::Atom(Atom(n)) = arg.as_ref() {
                    product *= n;
                } else {
                    return Value::Nil;
                }
            }
            Value::Atom(Atom(product))
        }),
        (b'/', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(n)) = args[0].as_ref() {
                if let Value::Atom(Atom(m)) = args[1].as_ref() {
                    Value::Atom(Atom(n / m))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }),
        (b'=', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a == b {
                        Value::T
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }),
        (b'<', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a < b {
                        Value::T
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }),
        (b'>', |args| {
            assert_eq!(args.len(), 2);
            if let Value::Atom(Atom(a)) = args[0].as_ref() {
                if let Value::Atom(Atom(b)) = args[1].as_ref() {
                    if a > b {
                        Value::T
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        })
    );
}
