use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_path_normalizer;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::collections::HashSet;

// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        let _ = utility_path_normalizer::extract_layer_from_prefix("");
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let root_dir_str = root_dir.to_string();

        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes201_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };
                let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
                if import_lines.is_empty() {
                    return Vec::new();
                }

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
                        self._check_forbidden_imports_with_lines(
                            &f_str,
                            &specialized,
                            def,
                            &import_lines,
                            &root_dir_str,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_forbidden_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        results.values.extend(file_violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportForbiddenChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_forbidden_imports_with_lines(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        import_lines: &[(
            shared::taxonomy_common_vo::LineNumber,
            shared::taxonomy_layer_vo::LineContentVO,
        )],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }
        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec!["agent".into(), "capabilities".into()]
        };

        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in import_lines {
            let module = match utility_import_resolver::extract_module_from_line(line) {
                Some(m) => m,
                None => continue,
            };
            let module_val = module.value();

            // ── Extract symbol name from import line for barrel resolution ──
            let symbol_name = extract_symbol_from_import_line((line_num, line));

            // ── Barrel resolution: resolve through __init__.py / mod.rs / index.ts ──
            // ── Fallback to filesystem scan if barrel resolution fails ──
            let resolved_layer = if let Some(sym) = &symbol_name {
                utility_import_resolver::resolve_barrel_import(module_val, sym, root_dir)
                    .and_then(|r| r.resolved_layer)
                    .or_else(|| utility_layer_detector::resolve_module_path_to_layer(module_val, root_dir))
            } else {
                None
            };

            for forbidden in &forbidden_list {
                let forbidden_identity = Identity::new(forbidden);
                let (layer, suffixes) =
                    utility_import_resolver::resolve_scope(&forbidden_identity);

                // Use resolved layer from barrel if available, otherwise use original module
                let check_val = resolved_layer.clone().unwrap_or_else(|| module_val.to_string());
                let is_forbidden = if suffixes.is_empty() {
                    check_val
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .any(|seg| {
                            let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                            match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                } else {
                    utility_import_resolver::import_matches_scope(line, &layer, &suffixes)
                };
                if is_forbidden {
                    let allowed: Vec<LayerNameVO> = definition
                        .allowed
                        .values
                        .iter()
                        .map(|s| {
                            LayerNameVO::new(
                                utility_import_resolver::resolve_scope(&Identity::new(s))
                                    .0
                                    .value()
                                    .to_string(),
                            )
                        })
                        .collect();
                    violations.push(LintResult::new_arch(
                        file,
                        line_num.value() as usize,
                        "AES201",
                        Severity::CRITICAL,
                        AesImportViolation::ForbiddenImport {
                            source_layer: layer_name_vo.clone(),
                            forbidden_layer: LayerNameVO::new(forbidden.clone()),
                            allowed,
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }

    fn _check_scope_forbidden_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(
            shared::taxonomy_common_vo::LineNumber,
            shared::taxonomy_layer_vo::LineContentVO,
        )],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &Identity::new(&rule.scope.value),
                )
            else {
                continue;
            };

            for (line_num, line) in import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let module_val = module.value();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            module_val
                                .split([':', '.', '/', '\\'])
                                .filter(|s| !s.is_empty())
                                .any(|seg| {
                                    let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                    match utility_import_resolver::extract_layer_from_import(
                                        &cleaned,
                                    ) {
                                        Some(l) => l == forbidden_layer,
                                        None => false,
                                    }
                                })
                        } else {
                            utility_import_resolver::import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        utility_import_resolver::resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: LayerNameVO::new(rule_layer_str.clone()),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: None,
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

/// Extract the symbol name from an import line for barrel resolution.
/// Returns None if no symbol can be extracted.
fn extract_symbol_from_import_line(
    line: (&shared::common::taxonomy_common_vo::LineNumber, &shared::common::taxonomy_layer_vo::LineContentVO),
) -> Option<String> {
    let trimmed = line.1.value().trim();

    // Python: from X import Y
    if let Some(rest) = trimmed.strip_prefix("from ") {
        if let Some(import_part) = rest.split_once(" import ") {
            let imports = import_part.1;
            // Handle "from X import Y, Z" → take first symbol
            for name in imports.split(',') {
                let name = name.trim().split(" as ").next().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    return Some(name.to_string());
                }
            }
        }
    }

    // Rust: use module::Type; or use module::{A, B};
    if trimmed.starts_with("use ")
        || trimmed.starts_with("pub use ")
        || trimmed.starts_with("pub(crate) use ")
    {
        let use_part = trimmed
            .trim_start_matches("pub(crate) use ")
            .trim_start_matches("pub use ")
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if let Some(brace_pos) = use_part.find("::{") {
            // use module::{A, B};
            let inner = use_part[brace_pos + 3..].trim_end_matches('}');
            for name in inner.split(',') {
                let name = name.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    return Some(name.to_string());
                }
            }
        } else {
            // use module::Type;
            let name = use_part.rsplit("::").next().unwrap_or("").trim();
            if !name.is_empty() && name != "*" {
                return Some(name.to_string());
            }
        }
    }

    // TS/JS: import { X, Y } from './module';
    if trimmed.starts_with("import ") && trimmed.contains(" from ") {
        if let Some(from_pos) = trimmed.rfind(" from ") {
            let import_part = &trimmed[7..from_pos];
            if import_part.contains('{') {
                if let Some(open) = import_part.find('{') {
                    if let Some(close) = import_part.find('}') {
                        let inner = &import_part[open + 1..close];
                        for name in inner.split(',') {
                            let name = name.trim().split(" as ").last().unwrap_or("").trim();
                            if !name.is_empty() {
                                return Some(name.to_string());
                            }
                        }
                    }
                }
            } else {
                // import X from './module'
                let name = import_part.trim();
                if !name.is_empty() && name != "default" {
                    return Some(name.to_string());
                }
            }
        }
    }

    None
}
