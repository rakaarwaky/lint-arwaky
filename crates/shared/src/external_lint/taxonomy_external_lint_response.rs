// PURPOSE: ExternalLintResponse — response payload for the external_lint aggregate

use shared_common::taxonomy_adapter_list_vo::AdapterNameList;
use shared_common::taxonomy_lint_result_vo::LintResultList;

pub enum ExternalLintResponse {
    /// Violations reported by the adapters.
    Scan { violations: LintResultList },
    /// Names of the registered adapters.
    AdapterNames { names: AdapterNameList },
}

impl ExternalLintResponse {
    pub fn into_violations(self) -> LintResultList {
        match self {
            Self::Scan { violations } => violations,
            Self::AdapterNames { .. } => LintResultList::default(),
        }
    }

    pub fn into_adapter_names(self) -> AdapterNameList {
        match self {
            Self::AdapterNames { names } => names,
            Self::Scan { .. } => AdapterNameList::default(),
        }
    }
}
