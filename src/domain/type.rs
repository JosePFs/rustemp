pub struct Types(pub String);

impl From<&str> for Types {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
