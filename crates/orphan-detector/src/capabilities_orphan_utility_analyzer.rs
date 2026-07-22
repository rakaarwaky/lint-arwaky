use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_file_handler as utility_file;
use shared::common::utility_file_handler;
use shared::common::utility_layer_detector;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

// Layers that are valid consumers of utility files
const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "root"];

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

        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        // Phase 1: Check import graph for consumer-layer importers
        if let Some(importers) = inbound_links.mapping.get(fp) {
            let external_importers: Vec<&String> = importers
                .iter()
                .filter(|importer| *importer != fp)
                .collect();

            if !external_importers.is_empty() {
                // Check if any importer is from a consumer layer (capability, agent, surface, root)
                let has_consumer = external_importers.iter().any(|importer| {
                    let filename = utility_layer_detector::extract_filename(importer);
                    utility_layer_detector::detect_layer_from_prefix(filename)
                        .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                        .unwrap_or(false)
                });

                if has_consumer {
                    // Utility is used by a consumer layer — not dead
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }

                // Utility is only imported by other utilities — dead code
                let importer_names: Vec<String> = external_importers
                    .iter()
                    .filter_map(|i| {
                        std::path::Path::new(i)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect();

                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::UtilityDeadCode {
                        stem: module_name.clone(),
                        imported_by: importer_names,
                        reason: Some(
                            format!(
                                "Utility file '{}' is only imported by other utility files, not by capability, agent, or surfaces layers.",
                                module_name
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        // Phase 2: Fallback — token-based matching across all files
        let tokens = shared::orphan_detector::utility_orphan_detector::import_tokens(fp);
        let mut consumer_importers: Vec<String> = Vec::new();
        let mut utility_importers: Vec<String> = Vec::new();

        for other_file in all_files {
            if other_file == fp {
                continue;
            }

            let other_content = utility_file::read_file_safe(other_file);
            if other_content.is_empty() {
                continue;
            }

            let is_consumer = {
                let filename = utility_layer_detector::extract_filename(other_file);
                utility_layer_detector::detect_layer_from_prefix(filename)
                    .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                    .unwrap_or(false)
            };

            let imported = self.check_import_pattern(&other_content, &module_name)
                || tokens.iter().any(|token| {
                    shared::orphan_detector::utility_orphan_detector::contains_delimited(
                        &other_content,
                        token,
                    )
                });

            if imported {
                let stem = std::path::Path::new(other_file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                if is_consumer {
                    consumer_importers.push(stem);
                } else {
                    utility_importers.push(stem);
                }
            }
        }

        // If imported by consumer layers — not dead
        if !consumer_importers.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // If only imported by other utilities — dead code
        if !utility_importers.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::UtilityDeadCode {
                    stem: module_name.clone(),
                    imported_by: utility_importers,
                    reason: Some(
                        format!(
                            "Utility file '{}' is only imported by other utility files, not by capability, agent, or surfaces layers.",
                            module_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        // Not imported by anyone — orphan
        OrphanIndicatorResult::new(
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
        )
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

    pub fn check_import_pattern(&self, content: &str, module_name: &str) -> bool {
        // Check for Rust use statements with the module name anywhere in the path
        // This handles:
        // - `use utility_target`
        // - `use utility_target::`
        // - `use crate::utility_target`
        // - `use shared::utility_target`
        // - `use shared::code_analysis::utility_target` (nested paths)
        // - `use crate::code_analysis::utility_target` (nested paths)
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ")
                || trimmed.starts_with("pub use ")
                || trimmed.starts_with("pub(crate) use ")
            {
                // Check if module_name appears as a path segment
                if self.path_contains_module(trimmed, module_name) {
                    return true;
                }
            }
        }

        // Check for grouped imports: use { module_name, ... }
        if content.contains(&format!("::{{{}}}", module_name))
            || content.contains(&format!("::{{{},", module_name))
            || content.contains(&format!(", {}::", module_name))
            || content.contains(&format!(", {}}}", module_name))
        {
            return true;
        }

        // Check for Python imports
        if content.contains(&format!("import {}", module_name))
            || content.contains(&format!("from {} import", module_name))
        {
            return true;
        }

        // Check for JavaScript/TypeScript imports
        if content.contains(&format!("from '{}'", module_name))
            || content.contains(&format!("from \"{}\"", module_name))
            || content.contains(&format!("require('{}')", module_name))
            || content.contains(&format!("require(\"{}\")", module_name))
        {
            return true;
        }

        false
    }

    /// Check if a use statement path contains the module name as a segment.
    fn path_contains_module(&self, use_statement: &str, module_name: &str) -> bool {
        // Extract the path part after "use " or "pub use " etc.
        let path = if let Some(pos) = use_statement.find("use ") {
            &use_statement[pos + 4..]
        } else {
            return false;
        };

        // Remove trailing semicolon and braces
        let path = path.trim_end_matches(';').trim();

        // Handle grouped imports: use path::{A, B, C}
        let path = if let Some(pos) = path.find("::{") {
            &path[..pos]
        } else {
            path
        };

        // Split by :: and check if any segment matches the module name
        let segments: Vec<&str> = path.split("::").collect();
        segments.contains(&module_name)
    }
}
