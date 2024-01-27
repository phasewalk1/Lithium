#[macro_export]
macro_rules! load_builtin_ops {
    ($map:expr, $( $id:expr => $func:expr ),* $(,)?) => {
        $(
            $map.insert(
                $id,
                Rc::new(crate::primitive::Operator {
                    id: $id,
                    f: crate::primitive::OperatorFunc($func),
                }),
            );
        )*
    };
}

#[macro_export]
macro_rules! load_builtin_keywords {
    ($map:expr, $( $id:expr => $func:expr ),* $(,)?) => {
        $(
            $map.insert(
                $id,
                Rc::new(crate::primitive::Keyword {
                    id: $id,
                }),
            );
        )*
    };
}
