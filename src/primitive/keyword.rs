#[derive(Debug)]
pub struct Keyword {
    pub id: String,
}

impl std::cmp::PartialEq for Keyword {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
