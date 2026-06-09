// arch_import_processor — Logic for evaluating architectural import rules.
// aes: wired-by-dispatch
// aes: bypass-capability-routing

use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

use crate::layer_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProcessorProtocol, ValidateImportsParams,
};
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_violationrs_constant::AES001_FORBIDDEN_IMPORT;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct ImportCheckerContext<'a> {
    pub imported_aliases: &'a std::collections::HashMap<String, String>,
    pub real_usages: &'a std::collections::HashSet<String>,
    pub class_bases: &'a std::collections::HashMap<String, Vec<String>>,
}

pub struct ArchImportProcessor {}

impl Default for ArchImportProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchImportProcessor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let layer_vo = match analyzer.detect_layer(file_path, root_dir) {
            Some(l) => l,
            None => return,
        };
        let definition = match analyzer.layer_map().values.get(&layer_vo) {
            Some(d) => d.clone(),
            None => return,
        };

        let file_path_str = file_path.to_string();
        let basename = Path::new(file_path_str.as_str())
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        if definition.exceptions.values.iter().any(|e| e == basename) {
            return;
        }

        if definition.forbidden.values.is_empty() && definition.allowed.values.is_empty() {
            return;
        }

        let imports = match analyzer.parser().extract_imports(file_path) {
            Ok(imp) => imp,
            Err(_) => return,
        };
        for imp in imports.values {
            self._evaluate_import(analyzer, &imp, file_path, &layer_vo, &definition, results);
        }
    }

    fn _evaluate_import(
        &self,
        analyzer: &dyn IAnalyzer,
        imp: &crate::code_analysis::taxonomy_import_source_vo::ImportInfo,
        file_path: &FilePath,
        file_layer: &LayerNameVO,
        definition: &LayerDefinition,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let module = FilePath::new(imp.module.clone())
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        let target_layer = match analyzer.detect_module_layer(&module) {
            Some(l) => l,
            None => return,
        };

        if !definition.allowed.values.is_empty() {
            let is_same = self._is_same_domain_layer(&target_layer, file_layer);
            let allowed = definition
                .allowed
                .values
                .iter()
                .any(|p| self._is_layer_match(&target_layer, p));
            if !is_same && !allowed {
                let msg = AES001_FORBIDDEN_IMPORT;
                self._add_forbidden_violation(
                    results,
                    file_path,
                    imp,
                    file_layer,
                    &target_layer,
                    msg,
                );
                return;
            }
        }

        if definition
            .forbidden
            .values
            .iter()
            .any(|p| self._is_layer_match(&target_layer, p))
        {
            self._add_forbidden_violation(
                results,
                file_path,
                imp,
                file_layer,
                &target_layer,
                AES001_FORBIDDEN_IMPORT,
            );
        }
    }

    fn _is_layer_match(&self, layer_vo: &LayerNameVO, pattern: &str) -> bool {
        let layer_name = &layer_vo.value;
        if layer_name == pattern {
            return true;
        }
        if layer_name.contains('(') {
            let base = layer_name.split('(').next().unwrap_or(layer_name);
            if pattern == base {
                return true;
            }
        }
        if pattern.contains('(') && layer_name.contains('(') {
            let p_base = pattern.split('(').next().unwrap_or(pattern);
            let p_subs_raw = pattern
                .split_once('(')
                .map(|x| x.1)
                .unwrap_or("")
                .trim_end_matches(')');
            let l_base = layer_name.split('(').next().unwrap_or(layer_name);
            let l_sub_raw = layer_name
                .split_once('(')
                .map(|x| x.1)
                .unwrap_or("")
                .trim_end_matches(')');
            if p_base != l_base {
                return false;
            }
            let p_subs: Vec<&str> = p_subs_raw.split('|').map(|s| s.trim()).collect();
            return p_subs.contains(&l_sub_raw);
        }
        false
    }

    fn _is_same_domain_layer(&self, layer_a: &LayerNameVO, layer_b: &LayerNameVO) -> bool {
        if layer_a == layer_b {
            return true;
        }
        let a_base = layer_a.value.split('(').next().unwrap_or(&layer_a.value);
        let b_base = layer_b.value.split('(').next().unwrap_or(&layer_b.value);
        a_base == b_base
    }

    fn _add_forbidden_violation(
        &self,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        file_path: &FilePath,
        imp: &crate::code_analysis::taxonomy_import_source_vo::ImportInfo,
        _layer_name: &LayerNameVO,
        _target_layer_name: &LayerNameVO,
        message: &str,
    ) {
        results.push(LintResult {
            file: file_path.clone(),
            line: imp.line.clone(),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES001"),
            message: LintMessage::new(message),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: None,
            related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
        });
    }

    pub async fn validate_imports_present(&self, params: ValidateImportsParams<'_>) {
        let symbols_data = match params.analyzer.parser().get_raw_symbols(params.file_path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let mut imported_aliases: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut used_symbols: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut class_bases: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        if let Some(val) = &symbols_data.value {
            if let Some(obj) = val.as_object() {
                if let Some(aliases) = obj.get("aliases").and_then(|a| a.as_object()) {
                    for (k, v) in aliases {
                        if let Some(v_str) = v.as_str() {
                            imported_aliases.insert(k.clone(), v_str.to_string());
                        }
                    }
                }
                if let Some(used) = obj.get("used").and_then(|u| u.as_array()) {
                    for sym in used {
                        if let Some(s) = sym.as_str() {
                            used_symbols.insert(s.to_string());
                        }
                    }
                }
                if let Some(bases) = obj.get("class_bases").and_then(|b| b.as_object()) {
                    for (k, v) in bases {
                        if let Some(arr) = v.as_array() {
                            let strs: Vec<String> = arr
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            class_bases.insert(k.clone(), strs);
                        }
                    }
                }
            }
        }

        let real_usages: std::collections::HashSet<String> = used_symbols
            .iter()
            .filter(|n| !self._is_bypass_marker(n))
            .cloned()
            .collect();
        let mut found_layers: std::collections::HashSet<String> = std::collections::HashSet::new();

        for req_layer in &params.required_layers.values {
            let satisfied = if req_layer.starts_with("contract") {
                let import_ctx = ImportCheckerContext {
                    imported_aliases: &imported_aliases,
                    real_usages: &real_usages,
                    class_bases: &class_bases,
                };
                self._check_contract_layer(
                    req_layer,
                    &import_ctx,
                    params.file_path,
                    params.layer_name,
                    params.results,
                )
            } else {
                self._check_general_layer(
                    params.analyzer,
                    req_layer,
                    &imported_aliases,
                    &real_usages,
                )
            };
            if satisfied {
                found_layers.insert(req_layer.clone());
            }
        }

        let missing: Vec<String> = params
            .required_layers
            .values
            .iter()
            .filter(|r| !found_layers.contains(*r))
            .cloned()
            .collect();
        if !missing.is_empty() {
            self._report_missing_imports(
                params.results,
                params.file_path,
                params.layer_name,
                params.layers_display,
                &missing,
                &params.message_template.value,
            );
        }
    }

    fn _report_missing_imports(
        &self,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        file_path: &FilePath,
        layer_vo: &LayerNameVO,
        layers_display: &crate::shared_common::taxonomy_common_vo::PatternList,
        missing: &[String],
        template: &str,
    ) {
        let contract_missing = missing.iter().any(|m| m.starts_with("contract"));
        let suffix = if contract_missing {
            " [STRICT] 'contract' imports must be from a specific _port, _protocol, or _aggregate module and used as a base class. Bare 'import contract as contract' or bypass markers are forbidden."
        } else {
            ""
        };
        let layers_str = layers_display.values.join(", ");
        let missing_str = format!("[{}]", missing.join(", "));
        let message = format!("{}{}", template, suffix)
            .replace("{layer}", &layer_vo.value)
            .replace("{layers}", &layers_str)
            .replace("{missing}", &missing_str);

        results.push(LintResult {
            file: file_path.clone(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES002"),
            message: LintMessage::new(message),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::HIGH,
            enclosing_scope: None,
            related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
        });
    }

    fn _is_bypass_marker(&self, name: &str) -> bool {
        if (name.starts_with("_arch_") && name.ends_with("_marker")) || name == "_" {
            return true;
        }
        if name.starts_with('_') {
            let lower = name.to_lowercase();
            let bypass_keywords = [
                "marker",
                "stub",
                "compliance",
                "dummy",
                "fake",
                "bypass",
                "placeholder",
                "sentinel",
                "shim",
            ];
            if bypass_keywords.iter().any(|kw| lower.contains(kw)) {
                return true;
            }
        }
        false
    }

    fn _check_import_stem_matches(
        &self,
        aliases: &[String],
        imported_aliases: &std::collections::HashMap<String, String>,
        class_bases: &std::collections::HashMap<String, Vec<String>>,
        file_path: &FilePath,
    ) -> Vec<String> {
        let all_bases: std::collections::HashSet<String> = class_bases
            .values()
            .flat_map(|bases| bases.iter().cloned())
            .collect();

        let used_as_base: Vec<String> = aliases
            .iter()
            .filter(|a| {
                all_bases.contains(*a)
                    || imported_aliases
                        .get(*a)
                        .is_some_and(|v| all_bases.contains(v))
                    || all_bases.iter().any(|b| b.starts_with(&format!("{}.", a)))
            })
            .cloned()
            .collect();

        let is_utility = file_path.to_string().ends_with("_util.py")
            || file_path.to_string().ends_with("_visitor.py");

        if used_as_base.is_empty() {
            if class_bases.is_empty() || is_utility {
                return aliases.to_vec();
            }
            return vec![];
        }
        used_as_base
    }

    fn _check_contract_layer(
        &self,
        req_layer_str: &str,
        context: &ImportCheckerContext,
        file_path: &FilePath,
        layer_vo: &LayerNameVO,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) -> bool {
        let aliases = self._get_contract_barrel_aliases(
            context.imported_aliases,
            context.real_usages,
            file_path,
            layer_vo,
            results,
        );
        if aliases.is_empty() {
            return false;
        }
        let used_as_base = self._check_import_stem_matches(
            &aliases,
            context.imported_aliases,
            context.class_bases,
            file_path,
        );
        if used_as_base.is_empty() {
            return false;
        }

        static CAPTURE_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"contract\((.+)\)").ok());

        if let Some(caps) = CAPTURE_RE
            .as_ref()
            .and_then(|re| re.captures(req_layer_str))
        {
            let _pattern = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        }

        true
    }

    fn _get_contract_barrel_aliases(
        &self,
        imported_aliases: &std::collections::HashMap<String, String>,
        real_usages: &std::collections::HashSet<String>,
        _file_path: &FilePath,
        _layer_vo: &LayerNameVO,
        _results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) -> Vec<String> {
        let mut aliases = Vec::new();
        for (alias, fullname) in imported_aliases {
            let parts: Vec<&str> = if fullname.contains("::") {
                fullname.split("::").collect()
            } else {
                fullname.split('.').collect()
            };
            if !parts.contains(&"contract") {
                continue;
            }
            if alias != "contract" && real_usages.contains(alias) {
                aliases.push(alias.clone());
            }
        }
        aliases
    }

    fn _check_general_layer(
        &self,
        analyzer: &dyn IAnalyzer,
        req_layer: &str,
        imported_aliases: &std::collections::HashMap<String, String>,
        real_usages: &std::collections::HashSet<String>,
    ) -> bool {
        for (alias, fullname) in imported_aliases {
            let module = FilePath::new(fullname.clone())
                .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
            let detected = analyzer.detect_module_layer(&module);
            let layer_match = detected
                .as_ref()
                .is_some_and(|l| self._is_layer_match(l, req_layer));
            let segment_match = fullname.split('.').any(|s| s == req_layer);
            if (layer_match || segment_match) && real_usages.contains(alias) {
                return true;
            }
        }
        false
    }
}

#[async_trait]
impl IArchImportProcessorProtocol for ArchImportProcessor {
    async fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        self.process_file_imports(analyzer, file_path, root_dir, results)
            .await;
    }

    async fn validate_imports_present(&self, params: ValidateImportsParams<'_>) {
        self.validate_imports_present(params).await;
    }
}
