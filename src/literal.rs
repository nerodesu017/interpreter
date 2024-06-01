#[derive(Debug, Clone)]
pub enum Literal {
    // value: LiteralValue
    SString(String),
    Float(f64),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "")
        unimplemented!()
    }
}
