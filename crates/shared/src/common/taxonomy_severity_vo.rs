// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }

    /// The one severity badge used by every human-facing text surface.
    ///
    /// Before this existed the report formatter used an ASCII ladder
    /// (`[!!!]`, `[!! ]`, `[!  ]`, `[.  ]`, `[   ]`) whose meaning lived in a
    /// trailing space, the `scan` text output printed no severity at all, and
    /// the TUI printed a third vocabulary. A badge carries no whitespace, so
    /// copying it, grepping it, or pasting it into an issue keeps it intact.
    pub fn badge(&self) -> &'static str {
        match self {
            Severity::INFO => "[INFO]",
            Severity::LOW => "[LOW]",
            Severity::MEDIUM => "[MED]",
            Severity::HIGH => "[HIGH]",
            Severity::CRITICAL => "[CRIT]",
        }
    }
}
