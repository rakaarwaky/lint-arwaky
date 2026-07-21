// PURPOSE: UtilityOrphanAnalyzer — IUtilityOrphanProtocol for detecting orphan utility files
//
// Utility files contain stateless standalone functions that provide low-level
// technical mechanics. A utility file is orphaned if no other file imports it.
//
// ALGORITHM:
//   1. Read the utility file content.
//   2. Extract the module path that other files would use to import it.
//   3. Scan all other files for import statements referencing this module.
//   4. If no file imports it, mark as orphan.

use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UtilityOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match shared::orphan_detector::utility_orphan_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
            }
            c => c,
        };

        // Extract the module identifier that other files would use to import this utility.
        // For Rust: the module name (e.g., "utility_file" from "utility_file.rs")
        // For Python: the module name from the filename
        // For JS/TS: the module name from the filename
        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        // Check if any other file imports this utility module
        let mut imported = false;
        for other_file in all_files {
            // Skip the file itself
            if other_file == fp {
                continue;
            }

            let other_content =
                shared::orphan_detector::utility_orphan_io::read_file_safe(other_file);
            if other_content.is_empty() {
                continue;
            }

            // Check for various import patterns:
            // Rust: use crate::...::module_name, use shared::...::module_name
            // Python: from module_name import, import module_name
            // JS/TS: import { ... } from '...module_name', require('...module_name')
            if self.check_import_pattern(&other_content, &module_name) {
                imported = true;
                break;
            }
        }

        if !imported {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::UtilityOrphan {
                    stem: module_name.clone(),
                    reason: Some(
                        format!(
                            "Utility file '{}' is not imported by any other file.",
                            module_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UtilityOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if the content contains an import of the given module name.
    fn check_import_pattern(&self, content: &str, module_name: &str) -> bool {
        // Rust patterns: use ...::module_name, use ...::module_name::
        if content.contains(&format!("use {}", module_name))
            || content.contains(&format!("use {}::", module_name))
            || content.contains(&format!("use crate::{}", module_name))
            || content.contains(&format!("use shared::{}", module_name))
        {
            return true;
        }

        // Python patterns: import module_name, from module_name import
        if content.contains(&format!("import {}", module_name))
            || content.contains(&format!("from {} import", module_name))
        {
            return true;
        }

        // JS/TS patterns: import ... from '...module_name', require('...module_name')
        if content.contains(&format!("from '{}'", module_name))
            || content.contains(&format!("from \"{}\"", module_name))
            || content.contains(&format!("require('{}')", module_name))
            || content.contains(&format!("require(\"{}\")", module_name))
        {
            return true;
        }

        false
    }
}
