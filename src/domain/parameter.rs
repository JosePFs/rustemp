use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Parameter(pub String);

impl Parameter {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Parameter {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Parameter {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
