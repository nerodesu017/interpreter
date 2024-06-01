#[derive(Debug, Clone)]
pub enum Literal {
    SString(String),
    Float(f64),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Float(val) => {
                    val.to_string()
                }
                Literal::SString(val) => {
                    val.to_owned()
                }
            }
        )
    }
}
