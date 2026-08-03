// AES404: utility contains forbidden type definition (struct)
pub struct ParserConfig {
    pub max_depth: usize,
    pub strict: bool,
}

pub fn parse(input: &str) -> String {
    input.to_uppercase()
}
