// PURPOSE: CodeAnalysisOrchestrator — agent that orchestrates Code Quality (AES301–AES305) checks and file collection
// ALGORITHM (run_analysis_with_entries):
//   1. If config.enabled = false, return empty
//   2. Resolve the enabled state of each rule code (UX-S5-002): a rule listed in
//      config.rules with `enabled: false` is skipped; rules absent from config
//      keep their built-in default (enabled).
//   3. For each pre-parsed file entry (parallel via rayon):
//      a. Empty or unparseable files emit a single AES302 FILE_TOO_SHORT finding
//         (UX-S5-001) and skip the remaining rules for that file.
//      b. Run bypass_checker.check_bypass_comments (AES304 — layer-independent)
//      c. Run dead_inheritance_checker.check_dead_inheritance (AES303 sub-check 2)
//      d. Skip barrel files (mod.rs, __init__.py, index.ts)
//      e. Detect layer from filename prefix; skip if unknown or in exception list
//      f. Run line_checker.check_line_counts (AES301–302)
//      g. Run class_checker.check_mandatory_class_definition (AES303 sub-check 1)
//   4. Run duplication check using pre-read entries (AES305)
//   5. Return aggregated LintResult list
// NOTE: scoring, criticality, and report formatting live in shared utilities
// (compute_score / contains_critical_severity / format_code_analysis_violation)
// and are consumed directly by callers.
// NOTE: whole-file findings use WHOLE_FILE (0) as their line marker; the
// renderer prints "file" for line 0 (UX-S5-003).

use rayon::prelude::*;
use shared_cli_commands::LintResult;

use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared_quality_rules::contract_quality_protocol::IBypassCheckerProtocol;
use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::contract_quality_protocol::IDeadInheritanceProtocol;
use shared_quality_rules::contract_quality_protocol::ILineCheckerProtocol;
use shared_quality_rules::contract_quality_protocol::IMandatoryClassProtocol;
use shared_quality_rules::taxonomy_quality_rules_request::CodeAnalysisRequest;
use shared_quality_rules::taxonomy_quality_rules_response::CodeAnalysisResponse;
use shared_quality_rules::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;

use shared_common::taxonomy_severity_vo::Severity;
use shared_common::utility_architecture_layer_detector::{
    collect_layer_keys, detect_layer_from_prefix, extract_filename, get_layer_def,
    resolve_specialized_layer,
};
use shared_common::{LayerMapVO, LayerNameVO};
use shared_config_system::ArchitectureConfig;

use shared_quality_rules::utility_violation_formatter::format_code_analysis_violation;
use std::sync::Arc;

// ─── Block 1: Module Declaration ───────────────────────────

/// Whole-file line marker (UX-S5-003). Findings about a file as a whole —
/// AES301/AES302 line counts, AES303 missing definition, AES305 duplication
/// share — carry this line instead of an invented location; the renderer
/// prints "file" for line 0.
pub const WHOLE_FILE: usize = 0;

/// Per-rule enabled state for one analysis run.
///
/// Rules listed in `ArchitectureConfig.rules` with `enabled: false` are
/// skipped; rules absent from the config keep their built-in default
/// (enabled).
#[derive(Debug, Default)]
pub struct RuleEnabled {
    aes301: bool,
    aes302: bool,
    aes303: bool,
    aes304: bool,
    aes305: bool,
}

impl RuleEnabled {
    pub fn from_config(config: &ArchitectureConfig) -> Self {
        // Default: every rule enabled (built-in behaviour).
        let mut state = Self {
            aes301: true,
            aes302: true,
            aes303: true,
            aes304: true,
            aes305: true,
        };
        for rule in &config.rules {
            if !rule.enabled.value {
                match rule.name.value.as_str() {
                    "AES301" => state.aes301 = false,
                    "AES302" => state.aes302 = false,
                    "AES303" => state.aes303 = false,
                    "AES304" => state.aes304 = false,
                    "AES305" => state.aes305 = false,
                    _ => {}
                }
            }
        }
        state
    }

    /// Rules that were disabled by config, in stable order.
    pub fn skipped_names(&self) -> Vec<&'static str> {
        let mut skipped = Vec::new();
        if !self.aes301 {
            skipped.push("AES301");
        }
        if !self.aes302 {
            skipped.push("AES302");
        }
        if !self.aes303 {
            skipped.push("AES303");
        }
        if !self.aes304 {
            skipped.push("AES304");
        }
        if !self.aes305 {
            skipped.push("AES305");
        }
        skipped
    }
}

pub struct CodeAnalysisDeps {
    pub bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    pub dead_inheritance_checker: Arc<dyn IDeadInheritanceProtocol>,
    pub line_checker: Arc<dyn ILineCheckerProtocol>,
    pub class_checker: Arc<dyn IMandatoryClassProtocol>,
    pub duplication_checker: Arc<dyn ICodeMetricAnalyzerProtocol>,
}

pub struct CodeAnalysisOrchestrator {
    deps: CodeAnalysisDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
}

impl CodeAnalysisOrchestrator {
    pub fn new(deps: CodeAnalysisDeps, config: ArchitectureConfig, layer_map: LayerMapVO) -> Self {
        Self {
            deps,
            config,
            layer_map,
        }
    }
}

// ─── Block 2: Module Contract ──────────────
impl ICodeAnalysisAggregate for CodeAnalysisOrchestrator {
    fn execute(&self, request: CodeAnalysisRequest) -> CodeAnalysisResponse {
        match request {
            CodeAnalysisRequest::RunAnalysis { files } => CodeAnalysisResponse::Analysis {
                violations: self.run_analysis_with_entries(&files),
            },
            CodeAnalysisRequest::Name => CodeAnalysisResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Module Helpers ──────

impl CodeAnalysisOrchestrator {
    pub fn name(&self) -> &str {
        "quality-rules"
    }

    /// Run analysis on pre-parsed file entries from the filesystem crate.
    ///
    /// UX-S5-001: empty or unparseable entries produce a single AES302
    /// `FILE_TOO_SHORT` finding (when a minimum line count is configured) and
    /// skip the remaining rules for that file.
    ///
    /// UX-S5-002: each rule's own `enabled` flag is honoured; disabled rules
    /// are skipped and named in the summary returned alongside the
    /// violations.
    pub fn run_analysis_with_entries(
        &self,
        files: &[shared_filesystem::taxonomy_filesystem_vo::FileEntry],
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        // UX-S5-002: honour each rule's own `enabled` flag; rules absent from
        // the config keep their built-in default (enabled).
        let rule_enabled = RuleEnabled::from_config(&self.config);

        let mut violations: Vec<LintResult> = Vec::new();

        // Parallel per-file processing using FileEntry content directly
        let file_violations: Vec<Vec<LintResult>> = files
            .par_iter()
            .map(|entry| {
                let mut v = Vec::new();
                let file = entry.path.to_string_lossy();
                let filename = std::path::Path::new(file.as_ref())
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                let c = &entry.content;

                // UX-S5-001: empty/unparseable files produce an AES302 finding
                // instead of being dropped before any rule runs. The finding
                // applies to the file as a whole, so it carries the WHOLE_FILE
                // (0) marker and the renderer prints "file" for that line.
                if !entry.parse_ok || c.is_empty() {
                    if rule_enabled.aes302 {
                        let base = detect_layer_from_prefix(filename);
                        let def = base.and_then(|base| {
                            let keys = collect_layer_keys(&self.layer_map);
                            let layer =
                                LayerNameVO::new(resolve_specialized_layer(&base, &file, &keys));
                            get_layer_def(&layer.value, &self.config.layers)
                        });
                        let min_lines = def
                            .as_ref()
                            .map(|d| d.code_analysis.min_lines.value)
                            .unwrap_or(0);
                        if min_lines > 0 {
                            v.push(LintResult::new_arch_with_name(
                                &file,
                                WHOLE_FILE,
                                "AES302",
                                Severity::HIGH,
                                format!("File is empty or unparseable, fewer than the minimum {min_lines} lines."),
                                "FILE_TOO_SHORT",
                                "A source file must contain real implementation so its behaviour is testable and its intent is explicit.",
                                "Expand the component or merge this logic into a related module.",
                            ));
                        }
                    }
                    return v;
                }

                // UX-S5-002: honour the AES304 rule's enabled flag
                if rule_enabled.aes304 {
                    self.deps
                        .bypass_checker
                        .check_bypass_comments(&file, c, &mut v);
                }

                // UX-S5-002: honour the AES303 rule's enabled flag
                if rule_enabled.aes303 {
                    self.deps
                        .dead_inheritance_checker
                        .check_dead_inheritance(&file, c, &mut v);
                }

                // AES304: Cargo.toml bypass detection
                if filename == "Cargo.toml" || filename == "cargo.toml" {
                    if rule_enabled.aes304 {
                        self.deps.bypass_checker.check_cargo_toml(c, &mut v);
                    }
                    return v;
                }

                // Skip barrel files (single source: shared_common::DEFAULT_RULE_EXCEPTIONS)
                if shared_common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) {
                    return v;
                }

                // Layer detection
                let fname = extract_filename(&file);
                let layer = match detect_layer_from_prefix(fname) {
                    Some(l) => l,
                    None => return v,
                };
                let keys = collect_layer_keys(&self.layer_map);
                let layer = LayerNameVO::new(resolve_specialized_layer(&layer, &file, &keys));
                let def = match get_layer_def(&layer.value, &self.config.layers) {
                    Some(d) => d,
                    None => return v,
                };
                if def.exceptions.values.contains(&fname.to_string()) {
                    return v;
                }

                // UX-S5-002: honour the AES301/AES302 rule's enabled flags.
                // The sub-check (too large vs too short) is picked by the
                // checker itself based on the layer's min/max configuration.
                if rule_enabled.aes301 || rule_enabled.aes302 {
                    self.deps
                        .line_checker
                        .check_line_counts(&file, Some(def), c, &mut v);
                }
                if rule_enabled.aes303 {
                    self.deps.class_checker.check_mandatory_class_definition(
                        &file,
                        Some(def),
                        c,
                        layer.value.as_str(),
                        &mut v,
                    );
                }

                v
            })
            .collect();

        for file_v in file_violations {
            violations.extend(file_v);
        }

        // AES305: duplication analysis on pre-fetched entries
        let entries: Vec<(std::path::PathBuf, String)> = files
            .iter()
            .filter(|f| f.parse_ok)
            .map(|f| (f.path.clone(), f.content.clone()))
            .collect();
        if rule_enabled.aes305 && !entries.is_empty() {
            // Hoist AES305 rule lookup outside per-violation loop (was O(violations) per scan)
            let aes305_rule = self.config.rules.iter().find(|r| r.name.value == "AES305");
            let dup_violations: Vec<(String, AesCodeAnalysisViolation)> = self
                .deps
                .duplication_checker
                .handle_duplicates_entries(&entries);
            for (file_path, aes_violation) in dup_violations {
                // Check AES305 exceptions (match against file stem or full filename)
                let file_name = std::path::Path::new(&file_path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let file_stem = std::path::Path::new(&file_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let excluded = if let Some(rule) = aes305_rule
                    && (rule.exceptions.values.contains(&file_name.to_string())
                        || rule.exceptions.values.contains(&file_stem.to_string()))
                {
                    true
                } else {
                    false
                };
                if excluded {
                    continue;
                }
                let msg = format_code_analysis_violation(&aes_violation);
                let vname =
                    shared_common::taxonomy_violation_item_vo::parse_violation_name("AES305", &msg);
                let why = shared_common::taxonomy_violation_item_vo::parse_why(&msg);
                let fix = shared_common::taxonomy_violation_item_vo::parse_fix(&msg);
                violations.push(LintResult::new_arch_with_name(
                    &file_path,
                    WHOLE_FILE,
                    "AES305",
                    Severity::MEDIUM,
                    msg,
                    &vname,
                    why,
                    fix,
                ));
            }
        }

        violations
    }
}
