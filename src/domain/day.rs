use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Day(pub String);

impl Day {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Day {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
