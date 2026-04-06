use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct LocationIds(pub String);

impl From<String> for LocationIds {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for LocationIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
