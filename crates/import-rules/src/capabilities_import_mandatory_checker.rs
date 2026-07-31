use async_trait::async_trait;
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::utility_layer_detector;
use shared::common::{
    FileContentVO, FilePath, FilePathList, Identity, LineContentVO, LineNumber, LintMessage,
    Severity, SymbolName,
};

use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use std::collections::HashSet;

// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.
//
// Barrel resolution: when direct module-path matching fails (e.g. import
// through __init__.py / mod.rs / index.ts hides the original file name),
// resolves each imported symbol through the barrel file to detect the
// original source file and its layer prefix.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    async fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
    ) -> Result<LintResultList, ImportError> {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let root_str = root_dir.value().to_string();

        let mut file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes202_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content =
                    match filesystem::utility_io::read_file(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };
                let file_content = FileContentVO::new(content);
                let import_lines: Vec<(LineNumber, LineContentVO)> =
                    utility_import_resolver::parse_import_lines_helper(
                        &f_str,
                        file_content.value(),
                    );

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer,
                        &f_str,
                        &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_mandatory_imports_with_lines(
                            &f_str,
                            &basename,
                            def,
                            &import_lines,
                            &root_str,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_mandatory_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &root_str,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        // Deduplicate violations by (file, line, code)
        file_violations.dedup_by(|a, b| a.file == b.file && a.line == b.line && a.code == b.code);

        Ok(LintResultList::new(file_violations))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() || basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let stem: &str = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer: &str = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let layer_str: &str = layer.value();

            // ── Direct match (existing behavior) ──
            let is_present_direct = import_lines
                .iter()
                .any(|(_, l)| utility_import_resolver::import_matches_scope(l, &layer, &suffixes));

            // ── Barrel resolution fallback ──
            let is_present = is_present_direct
                || self._check_barrel_mandatory_imports(
                    import_lines,
                    &layer,
                    &suffixes,
                    layer_str,
                    root_dir,
                );

            if !is_present {
                let v = LintResult::new_arch(
                    file,
                    1,
                    "AES202",
                    Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: Some(LintMessage::new(format!(
                            "File '{}' in layer '{}' is missing required import '{}'.",
                            basename, source_layer, required
                        ))),
                    }
                    .to_string(),
                );
                if !violations.contains(&v) {
                    violations.push(v);
                }
            }
        }
    }

    fn _check_scope_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.name.value != "AES202" && rule.rule_type.code() != "AES202" {
                continue;
            }

            // 1. Direct rule mandatory
            if !rule.mandatory.values.is_empty() {
                let scope_identity = Identity::new(&rule.scope.value);
                if let Some((rule_layer_str, _rule_suffixes)) =
                    shared::common::utility_scope_matcher::file_belongs_to_scope(
                        basename,
                        &scope_identity,
                    )
                {
                    for required in &rule.mandatory.values {
                        self._check_single_scope_requirement(
                            file,
                            basename,
                            &rule_layer_str,
                            required,
                            import_lines,
                            root_dir,
                            violations,
                        );
                    }
                }
            }
        }
    }

    fn _check_single_scope_requirement(
        &self,
        file: &str,
        basename: &str,
        rule_layer_str: &str,
        required: &str,
        import_lines: &[(LineNumber, LineContentVO)],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let required_identity = Identity::new(required);
        let (req_layer, req_suffixes) = utility_import_resolver::resolve_scope(&required_identity);
        let req_layer_str = req_layer.value();

        let is_present_direct = import_lines.iter().any(|(_, l)| {
            utility_import_resolver::import_matches_scope(l, &req_layer, &req_suffixes)
        });

        let is_present = is_present_direct
            || self._check_barrel_mandatory_imports(
                import_lines,
                &req_layer,
                &req_suffixes,
                req_layer_str,
                root_dir,
            );

        if !is_present {
            let v = LintResult::new_arch(
                file,
                1,
                "AES202",
                Severity::HIGH,
                AesImportViolation::MissingImport {
                    source_layer: LayerNameVO::new(rule_layer_str.to_string()),
                    required: SymbolName::new(required.to_string()),
                    reason: Some(LintMessage::new(format!(
                        "File '{}' in scope '{}' is missing required import '{}'.",
                        basename, rule_layer_str, required
                    ))),
                }
                .to_string(),
            );
            if !violations.contains(&v) {
                violations.push(v);
            }
        }
    }

    /// Barrel resolution fallback for mandatory import checks.
    ///
    /// When direct module-path matching fails (e.g. `from modules.shared.src.server import X`
    /// doesn't contain "contract"), resolve each imported symbol through the barrel file
    /// (__init__.py / mod.rs / index.ts) to find the original source file and its layer.
    ///
    /// Resolves via `utility_import_resolver::resolve_barrel_import()` which uses the shared
    /// `parse_barrel_reexports()` to track symbol → file_stem mappings.
    ///
    /// # Example
    /// ```text
    /// import:  from modules.shared.src.server import IBlenderConnectionProtocol
    /// barrel:  modules/shared/src/server/__init__.py
    ///          → from .contract_connection_protocol import IBlenderConnectionProtocol
    /// resolved: resolve_barrel_import → ResolvedImport {
    ///     resolved_file: "contract_connection_protocol",
    ///     resolved_layer: Some("contract"),
    /// }
    /// suffix "_protocol" check: contract_connection_protocol contains "_protocol" ✅
    /// ```
    fn _check_barrel_mandatory_imports(
        &self,
        import_lines: &[(LineNumber, LineContentVO)],
        _layer: &LayerNameVO,
        suffixes: &[Identity],
        layer_str: &str,
        root_dir: &str,
    ) -> bool {
        for (_, line) in import_lines {
            let line_val = line.value();

            // Extract module and symbol from raw_path (e.g. "shared::common::FilePath")
            let (module_val, symbol_name): (String, String) = match line_val.rsplit_once("::") {
                Some((m, s)) => (m.to_string(), s.to_string()),
                None => {
                    let symbols = utility_import_resolver::extract_symbol_names(line_val);
                    let module = utility_import_resolver::extract_module_from_line(line);
                    let mod_str = module
                        .as_ref()
                        .map(|m| m.value().to_string())
                        .unwrap_or_default();
                    if mod_str.is_empty() || symbols.is_empty() {
                        continue;
                    }
                    let Some(first_sym) = symbols.into_iter().next() else {
                        continue;
                    };
                    (mod_str, first_sym)
                }
            };

            if symbol_name.is_empty() || symbol_name == "*" {
                continue;
            }

            // Resolve symbol through barrel file to find original source file stem
            let Some(resolved) =
                utility_import_resolver::resolve_barrel_import(&module_val, &symbol_name, root_dir)
            else {
                continue;
            };

            // ── Layer must match the required layer ──
            if !resolved.matches_layer(layer_str) {
                continue;
            }

            // ── Suffix check (if required) ──
            if suffixes.is_empty() {
                return true;
            }
            if suffixes.iter().any(|s| resolved.has_suffix(s.value())) {
                return true;
            }
        }
        false
    }
}
