#[derive(Debug, Clone, PartialEq)]
pub enum OperationVO {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl OperationVO {
    pub fn symbol(&self) -> &str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
        }
    }

    pub fn from_symbol(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" | "x" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            _ => None,
        }
    }
}
