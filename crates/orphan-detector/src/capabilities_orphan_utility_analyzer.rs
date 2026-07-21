use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
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
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let fp = f.value();

        // Fast path: use already-built import graph
        if let Some(importers) = inbound_links.mapping.get(fp) {
            if importers.iter().any(|importer| importer != fp) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        // Fallback: token-based matching
        let tokens = shared::orphan_detector::utility_orphan::import_tokens(fp);
        let mut imported = false;
        for other_file in all_files {
            if other_file == fp {
                continue;
            }

            let other_content =
                shared::orphan_detector::utility_orphan_io::read_file_safe(other_file);
            if other_content.is_empty() {
                continue;
            }

            if self.check_import_pattern(&other_content, &module_name) {
                imported = true;
                break;
            }
            if tokens.iter().any(|token| {
                shared::orphan_detector::utility_orphan::contains_delimited(&other_content, token)
            }) {
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
                Severity::MEDIUM,
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

    fn check_import_pattern(&self, content: &str, module_name: &str) -> bool {
        if content.contains(&format!("use {}", module_name))
            || content.contains(&format!("use {}::", module_name))
            || content.contains(&format!("use crate::{}", module_name))
            || content.contains(&format!("use shared::{}", module_name))
        {
            return true;
        }

        if content.contains(&format!("import {}", module_name))
            || content.contains(&format!("from {} import", module_name))
        {
            return true;
        }

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
