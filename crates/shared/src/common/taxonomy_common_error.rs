// PURPOSE: Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::string_value_object;
use serde::Serialize;

string_value_object!(Cause);
string_value_object!(Constraint);
string_value_object!(FieldName);
string_value_object!(ModuleName);
string_value_object!(PrimitiveTypeName);

/// Strongly-typed exit code value object. Written manually because the
/// `string_value_object!` macro only supports `String` (not `i64`).
///
/// Workspace Exit Code Contract (root PRD):
///   0 = Ok / clean / diagnostic completed
///   1 = Policy fail (violations, CI fail, vulns found, remaining after fix)
///   2 = Runtime error (bad path, pipeline crash, invalid state)
///   3 = Prerequisite missing (required external tool not installed)
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: crate::common::taxonomy_common_vo::LineNumber,
}

impl ExitCode {
    pub fn new(value: impl Into<crate::common::taxonomy_common_vo::LineNumber>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value.value()
    }

    // ── Named constants (workspace exit-code contract) ──────────────
    /// Exit 0 — Ok / clean / diagnostic completed.
    pub const OK: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 0 },
    };
    /// Exit 1 — Policy fail (violations, CI fail, vulns found, remaining after fix).
    pub const POLICY_FAIL: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 1 },
    };
    /// Exit 2 — Runtime error (bad path, pipeline crash, invalid state).
    pub const RUNTIME_ERROR: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 2 },
    };
    /// Exit 3 — Prerequisite missing (required external tool not installed).
    pub const PREREQUISITE_MISSING: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 3 },
    };

    /// Convert to `std::process::ExitCode` for CLI surface return values.
    pub fn to_process_exit_code(&self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.value.value() as u8)
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self {
            value: crate::common::taxonomy_common_vo::LineNumber::new(v),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct W {
            value: crate::common::taxonomy_common_vo::LineNumber,
        }
        let w = W::deserialize(deserializer)?;
        Ok(Self { value: w.value })
    }
}
