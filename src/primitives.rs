#[derive(Debug, PartialEq)]
pub(super) struct Atom(pub i32);

#[derive(PartialEq)]
pub(super) struct Nil(());

#[derive(PartialEq)]
pub(super) struct SymbolicId([u8; 32]);

impl SymbolicId {
    pub(super) fn make_symbolic_id(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s.len() {
            0..=32 => {
                let mut id = [0; 32];
                id[..s.len()].copy_from_slice(s.as_bytes());
                Ok(Self(id))
            }
            _ => Err("SymbolicId must be 32 bytes or less".into()),
        }
    }

    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for SymbolicId {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() > 32 {
            Err("SymbolicId must be 32 bytes or less")
        } else {
            let mut id = [0; 32];
            id[..s.len()].copy_from_slice(s.as_bytes());
            Ok(Self(id))
        }
    }
}

#[derive(PartialEq)]
pub(super) struct Function {
    sid: Option<SymbolicId>,
    oid: Option<u8>,
    f: fn(&[Value]) -> Value,
}

impl Function {
    pub(super) fn new(id: SymbolicId, f: fn(&[Value]) -> Value) -> Self {
        Self {
            sid: Some(id),
            oid: None,
            f,
        }
    }

    pub(super) fn new_operator(id: u8, f: fn(&[Value]) -> Value) -> Self {
        Self {
            sid: None,
            oid: Some(id),
            f,
        }
    }

    pub(super) fn apply(&self, args: &[Value]) -> Value {
        (self.f)(args)
    }
}

// 'Nil' is a not a value
#[derive(PartialEq)]
pub(super) enum Value {
    Atom(Atom),
    Function(Function),
}
