use crate::environment::identifiers::SymbolId;

#[derive(Debug, PartialEq)]
pub(super) struct Symbol {
    id: SymbolId,
    value: super::Value,
}
