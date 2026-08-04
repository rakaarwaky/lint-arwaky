// PURPOSE: ConfigLanguage — typed enum for supported languages, prevents path injection via free-form strings
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigLanguage {
    Rust,
    Python,
    TypeScript,
}

impl ConfigLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigLanguage::Rust => "rust",
            ConfigLanguage::Python => "python",
            ConfigLanguage::TypeScript => "typescript",
        }
    }

    pub fn config_file_names(&self) -> &'static [&'static str] {
        &["lint_arwaky.config.yaml"]
    }
}

impl std::fmt::Display for ConfigLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConfigLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "javascript" | "js" => Ok(Self::TypeScript),
            other => Err(format!(
                "Unsupported language '{other}'. Supported: rust, python, typescript"
            )),
        }
    }
}
