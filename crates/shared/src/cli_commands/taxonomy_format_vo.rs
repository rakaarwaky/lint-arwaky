// PURPOSE: Format — output format enum for --format CLI arg (text, json, sarif, junit)
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Text,
    Json,
    Sarif,
    Junit,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Json => write!(f, "json"),
            Format::Sarif => write!(f, "sarif"),
            Format::Junit => write!(f, "junit"),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            "sarif" => Ok(Format::Sarif),
            "junit" => Ok(Format::Junit),
            other => Err(format!(
                "unknown format '{other}': expected one of text, json, sarif, junit"
            )),
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Text, Format::Json, Format::Sarif, Format::Junit]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Format::Text => Some(clap::builder::PossibleValue::new("text")),
            Format::Json => Some(clap::builder::PossibleValue::new("json")),
            Format::Sarif => Some(clap::builder::PossibleValue::new("sarif")),
            Format::Junit => Some(clap::builder::PossibleValue::new("junit")),
        }
    }
}
