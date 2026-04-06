use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Lang(pub String);

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Lang {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self("gl".to_string())
    }
}
