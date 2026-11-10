use shared_cli_commands::LintResult;
use shared_quality_rules::contract_quality_protocol::ILineCheckerProtocol;

use super::agent_quality_orchestrator::WHOLE_FILE;
use shared_common::taxonomy_layer_vo::LayerDefinition;
use shared_common::taxonomy_severity_vo::Severity;

// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
// ALGORITHM:
//   1. Skip barrel files (mod.rs, __init__.py)
//   2. If no LayerDefinition provided, skip
//   3. Check if filename is in exception list
//   4. Count lines in passed content string
//   5. If min_lines > 0 and count < min_lines → AES302 FILE_TOO_SHORT
//   6. If max_lines > 0 and count > max_lines → AES301 FILE_TOO_LARGE
use std::path::Path;

// ─── Block 1: Module Declaration ───────────────────────────

pub struct ArchLineChecker {}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ───────────────

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        // Skip barrel files (single source: shared_common::DEFAULT_RULE_EXCEPTIONS)
        if shared_common::DEFAULT_RULE_EXCEPTIONS.contains(&basename.as_str()) {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let count = content.lines().count() as i64;

        if def.code_analysis.min_lines.value > 0 && count < def.code_analysis.min_lines.value {
            violations.push(LintResult::new_arch_with_name(
                file,
                WHOLE_FILE,
                "AES302",
                Severity::HIGH,
                format!(
                    "File has {count} lines, fewer than the minimum {} lines.",
                    def.code_analysis.min_lines.value
                ),
                "FILE_TOO_SHORT",
                "A source file must carry enough implementation to be meaningful on its own; a stub invites logic to be scattered across call sites.",
                "Expand the component or merge this logic into a related module.",
            ));
        }

        if def.code_analysis.max_lines.value > 0 && count > def.code_analysis.max_lines.value {
            violations.push(LintResult::new_arch_with_name(
                file,
                WHOLE_FILE,
                "AES301",
                Severity::HIGH,
                format!(
                    "File has {count} lines, more than the maximum {} lines.",
                    def.code_analysis.max_lines.value
                ),
                "FILE_TOO_LARGE",
                "One bloated file hides its responsibilities and makes review, testing, and focused refactoring harder.",
                "Split the module into smaller, more focused files.",
            ));
        }
    }
}

// ─── Block 3: Module Helpers ───
