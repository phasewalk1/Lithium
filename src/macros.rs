#[macro_export]
macro_rules! ops_builtin {
    ($table:ident, $(($op:expr, $func:expr)),*) => {
        $(
            OperatorTable::add_operator(&mut $table.operators, $op, $func).unwrap();
        )*
    };
}
