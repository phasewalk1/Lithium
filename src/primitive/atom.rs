#[derive(Debug, PartialEq, PartialOrd)]
pub struct Atom(pub i32);

impl Atom {
    pub fn wrap(&self) -> super::Value {
        super::Value::Atom(Atom(self.0))
    }
}
