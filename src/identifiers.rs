pub trait Identifier {
    fn id(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq)]
pub struct SymbolId([u8; 32]);

impl TryFrom<&str> for SymbolId {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.len() {
            0..=32 => {
                let mut id = [0; 32];
                id[..s.len()].copy_from_slice(s.as_bytes());
                Ok(Self(id))
            }
            _ => Err("SymbolId must be 32 bytes or less".into()),
        }
    }
}

impl Identifier for SymbolId {
    fn id(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Identifier for u8 {
    fn id(&self) -> Vec<u8> {
        vec![*self]
    }
}
