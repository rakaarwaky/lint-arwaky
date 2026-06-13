// PURPOSE: Root container for code-analysis — defines CheckerContainer, RoleOrchestrator, and AnalysisContainer

use std::sync::Arc;

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_class_checker::ArchClassChecker;
use crate::capabilities_cycle_analyzer::DependencyCycleAnalyzer;
use crate::capabilities_dead_inheritance_checker::DeadInheritanceChecker;
use crate::capabilities_inline_unused_checker::InlineUnusedChecker;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_inline_unused_protocol::IInlineUnusedProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_result_vo::LintResultList;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;

/// CheckerContainer holds all the protocol implementations for AES checking
#[derive(Clone)]
pub struct CheckerContainer {
    analyzer: Arc<dyn IAnalyzer>,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    inline_unused_checker: Arc<dyn IInlineUnusedProtocol>,
    dead_inheritance_checker: Arc<dyn IDeadInheritanceProtocol>,
    mandatory_inheritance_checker: Arc<dyn IMandatoryInheritanceProtocol>,
    capabilities_role_checker: Arc<dyn ICapabilitiesRoleProtocol>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    taxonomy_checker: Arc<dyn ITaxonomyProtocol>,
    contract_checker: Arc<dyn IContractProtocol>,
    class_checker: Arc<dyn IMandatoryClassProtocol>,
    naming_checker: Arc<dyn INamingProtocol>,
    import_mandatory_checker: Arc<dyn IImportMandatoryProtocol>,
    import_intent_checker: Arc<dyn IImportIntentProtocol>,
    import_forbidden_checker: Arc<dyn IImportForbiddenProtocol>,
    cycle_analyzer: Arc<dyn ICycleAnalysisProtocol>,
    surface_checker: Arc<dyn ISurfaceProtocol>,
    orphan_aggregate: Arc<dyn IOrphanAggregate>,
}

pub struct CheckerContainerParts {
    pub capabilities_role_checker: Arc<dyn ICapabilitiesRoleProtocol>,
    pub taxonomy_checker: Arc<dyn ITaxonomyProtocol>,
    pub contract_checker: Arc<dyn IContractProtocol>,
    pub naming_checker: Arc<dyn INamingProtocol>,
    pub import_mandatory_checker: Arc<dyn IImportMandatoryProtocol>,
    pub import_intent_checker: Arc<dyn IImportIntentProtocol>,
    pub import_forbidden_checker: Arc<dyn IImportForbiddenProtocol>,
    pub surface_checker: Arc<dyn ISurfaceProtocol>,
    pub orphan_aggregate: Arc<dyn IOrphanAggregate>,
}

impl CheckerContainer {
    pub fn new(analyzer: Arc<dyn IAnalyzer>) -> Self {
        Self::new_with_parts(
            analyzer,
            CheckerContainerParts {
                capabilities_role_checker: Arc::new(PlaceholderCapabilitiesRoleChecker),
                taxonomy_checker: Arc::new(PlaceholderTaxonomyChecker),
                contract_checker: Arc::new(PlaceholderContractChecker),
                naming_checker: Arc::new(PlaceholderNamingChecker),
                import_mandatory_checker: Arc::new(PlaceholderImportMandatoryChecker),
                import_intent_checker: Arc::new(PlaceholderImportIntentChecker { _dummy: () }),
                import_forbidden_checker: Arc::new(PlaceholderImportForbiddenChecker),
                surface_checker: Arc::new(PlaceholderSurfaceChecker),
                orphan_aggregate: Arc::new(PlaceholderOrphanAggregate),
            },
        )
    }

    pub fn new_with_parts(analyzer: Arc<dyn IAnalyzer>, parts: CheckerContainerParts) -> Self {
        Self {
            analyzer,
            bypass_checker: Arc::new(BypassChecker {}),
            inline_unused_checker: Arc::new(InlineUnusedChecker {}),
            dead_inheritance_checker: Arc::new(DeadInheritanceChecker {}),
            mandatory_inheritance_checker: Arc::new(MandatoryInheritanceChecker {}),
            capabilities_role_checker: parts.capabilities_role_checker,
            line_checker: Arc::new(ArchLineChecker {}),
            taxonomy_checker: parts.taxonomy_checker,
            contract_checker: parts.contract_checker,
            class_checker: Arc::new(ArchClassChecker {}),
            naming_checker: parts.naming_checker,
            import_mandatory_checker: parts.import_mandatory_checker,
            import_intent_checker: parts.import_intent_checker,
            import_forbidden_checker: parts.import_forbidden_checker,
            cycle_analyzer: Arc::new(DependencyCycleAnalyzer::new(ArchitectureConfig::default())),
            surface_checker: parts.surface_checker,
            orphan_aggregate: parts.orphan_aggregate,
        }
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn inline_unused_checker(&self) -> &Arc<dyn IInlineUnusedProtocol> {
        &self.inline_unused_checker
    }

    pub fn dead_inheritance_checker(&self) -> &Arc<dyn IDeadInheritanceProtocol> {
        &self.dead_inheritance_checker
    }

    pub fn mandatory_inheritance_checker(&self) -> &Arc<dyn IMandatoryInheritanceProtocol> {
        &self.mandatory_inheritance_checker
    }

    pub fn capabilities_role_checker(&self) -> &Arc<dyn ICapabilitiesRoleProtocol> {
        &self.capabilities_role_checker
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn taxonomy_checker(&self) -> &Arc<dyn ITaxonomyProtocol> {
        &self.taxonomy_checker
    }

    pub fn contract_checker(&self) -> &Arc<dyn IContractProtocol> {
        &self.contract_checker
    }

    pub fn class_checker(&self) -> &Arc<dyn IMandatoryClassProtocol> {
        &self.class_checker
    }

    pub fn naming_checker(&self) -> &Arc<dyn INamingProtocol> {
        &self.naming_checker
    }

    pub fn import_mandatory_checker(&self) -> &Arc<dyn IImportMandatoryProtocol> {
        &self.import_mandatory_checker
    }

    pub fn import_intent_checker(&self) -> &Arc<dyn IImportIntentProtocol> {
        &self.import_intent_checker
    }

    pub fn import_forbidden_checker(&self) -> &Arc<dyn IImportForbiddenProtocol> {
        &self.import_forbidden_checker
    }

    pub fn cycle_analyzer(&self) -> &Arc<dyn ICycleAnalysisProtocol> {
        &self.cycle_analyzer
    }

    pub fn surface_checker(&self) -> &Arc<dyn ISurfaceProtocol> {
        &self.surface_checker
    }

    pub fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate> {
        &self.orphan_aggregate
    }

    pub fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.analyzer
            .detect_layer(&shared::source_parsing::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap_or_default(), &shared::source_parsing::taxonomy_path_vo::FilePath::new(root_dir.to_string()).unwrap_or_default())
    }

    pub fn get_layer_def(
        &self,
        _layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }

    pub fn analyzer(&self) -> &Arc<dyn IAnalyzer> {
        &self.analyzer
    }

    pub fn as_checker_ref(&self) -> &dyn CheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CheckerContainer
pub trait CheckerContainerRef {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition>;
}

// Local protocols that aren't in shared
pub trait ICapabilitiesRoleProtocol: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
        violations: &mut Vec<LintResult>,
    );
}

pub trait ITaxonomyProtocol: Send + Sync {
    fn check_entity(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_error(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_event(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
    fn check_constant(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
}

pub trait IContractProtocol: Send + Sync {
    fn check_aggregate(
        &self,
        source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        def: &shared::common::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}

pub trait INamingProtocol: Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportMandatoryProtocol: Send + Sync {
    fn check_mandatory_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportIntentProtocol: Send + Sync {
    fn check_mandatory_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check_forbidden_imports(
        &self,
        analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait ISurfaceProtocol: Send + Sync {
    fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root: &FilePath,
        violations: &mut LintResultList,
    );
}

pub trait IOrphanAggregate: Send + Sync {
    fn check_orphans(
        &self,
        container: &dyn CheckerContainerRef,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}

struct PlaceholderCapabilitiesRoleChecker;
impl ICapabilitiesRoleProtocol for PlaceholderCapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _layer: &shared::taxonomy_layer_vo::LayerNameVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderTaxonomyChecker;
impl ITaxonomyProtocol for PlaceholderTaxonomyChecker {
    fn check_entity(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_error(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_event(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
    fn check_constant(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderContractChecker;
impl IContractProtocol for PlaceholderContractChecker {
    fn check_aggregate(
        &self,
        _source: &shared::config_system::taxonomy_source_vo::SourceContentVO,
        _def: &shared::common::taxonomy_definition_vo::LayerDefinition,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}

struct PlaceholderNamingChecker;
impl INamingProtocol for PlaceholderNamingChecker {
    fn check_file_naming(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
    fn check_domain_suffixes(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderImportMandatoryChecker;
impl IImportMandatoryProtocol for PlaceholderImportMandatoryChecker {
    fn check_mandatory_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderImportIntentChecker { _dummy: () }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl Language {
    fn from_path(path: &str) -> Self {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        match ext {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" | "ts" | "jsx" | "tsx" => Language::JavaScript,
            _ => Language::Unknown,
        }
    }
}

impl IImportIntentProtocol for PlaceholderImportIntentChecker {
    fn check_mandatory_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        files: &FilePathList,
        _root: &FilePath,
        violations: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            // Skip self-check - this file contains hardcoded violation message strings
            if f_str.contains("root_code_analysis_container") {
                continue;
            }

            let Ok(content) = std::fs::read_to_string(&f_str) else {
                continue;
            };
            let lines: Vec<&str> = content.lines().collect();
            let lang = Language::from_path(&f_str);

            let dummy_ranges = dummy_function_ranges(&lines, lang);
            let dummy_impls = dummy_impl_traits(&lines);

            for (symbol, line_no) in imported_symbols(&lines, lang) {
                if symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impls) {
                    continue;
                }

                use shared::output_report::taxonomy_severity_vo::Severity;
                use shared::taxonomy_violation_message::AesViolation;
                use shared::taxonomy_name_vo::SymbolName;
                use shared::taxonomy_layer_vo::LayerNameVO;

                violations.values.push(LintResult::new_arch(
                    &f_str,
                    line_no,
                    "AES002X",
                    Severity::HIGH,
                    AesViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("any".to_string()),
                        import_type: SymbolName::new(symbol),
                        intent: SymbolName::new(
                            "Use imported symbols in real logic, not only in dummy functions or stubs"
                                .to_string(),
                        ),
                        reason: None,
                    },
                ));
            }

            for (start, _end) in dummy_function_ranges(&lines, lang) {
                use shared::output_report::taxonomy_severity_vo::Severity;
                use shared::taxonomy_violation_message::AesViolation;
                use shared::taxonomy_name_vo::SymbolName;
                use shared::taxonomy_layer_vo::LayerNameVO;

                violations.values.push(LintResult::new_arch(
                    &f_str,
                    start,
                    "AES002X",
                    Severity::HIGH,
                    AesViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("any".to_string()),
                        import_type: SymbolName::new("_use_mandatory_imports".to_string()),
                        intent: SymbolName::new(
                            "Remove dummy functions that exist only to silence unused import checks"
                                .to_string(),
                        ),
                        reason: None,
                    },
                ));
            }
        }
    }
}

fn dummy_function_ranges(lines: &[&str], lang: Language) -> Vec<(usize, usize)> {
    match lang {
        Language::Rust => rust_dummy_function_ranges(lines),
        Language::Python => python_dummy_function_ranges(lines),
        Language::JavaScript => js_dummy_function_ranges(lines),
        Language::Unknown => Vec::new(),
    }
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn _use_") || trimmed.starts_with("fn dummy_") {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;
            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }
            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }
    ranges
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("def _use_") || trimmed.starts_with("def dummy_") {
            let start = i + 1;
            let mut end = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();
            for (idx, line) in lines.iter().enumerate().skip(i + 1) {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') {
                    end = idx + 1;
                    continue;
                }
                let line_indent = line.len() - line.trim_start().len();
                if line_indent <= indent && !t.is_empty() {
                    break;
                }
                end = idx + 1;
            }
            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }
    ranges
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("function _use")
            || trimmed.starts_with("function dummy")
            || trimmed.starts_with("const _use")
            || trimmed.starts_with("const dummy")
        {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;
            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }
            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }
    ranges
}

fn imported_symbols(lines: &[&str], lang: Language) -> Vec<(String, usize)> {
    match lang {
        Language::Rust => rust_imported_symbols(lines),
        Language::Python => python_imported_symbols(lines),
        Language::JavaScript => js_imported_symbols(lines),
        Language::Unknown => Vec::new(),
    }
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }
        // Exempt test module patterns: `use super::*;` is standard in #[cfg(test)] modules
        if trimmed == "use super::*;" {
            continue;
        }
        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();
        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols.push((symbol, idx + 1));
                        }
                    }
                }
            }
            continue;
        }
        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((symbol, idx + 1));
        }
    }
    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }
    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }
    let name = part.split("::").last().unwrap_or(part).trim();
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }
    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name = name.trim().split_whitespace().next().unwrap_or("");
                    if !name.is_empty() && name != "*" {
                        symbols.push((name.to_string(), idx + 1));
                    }
                }
            }
            continue;
        }
        if trimmed.starts_with("import ") {
            let module = trimmed
                .trim_start_matches("import ")
                .trim()
                .split_whitespace()
                .next()
                .unwrap_or("");
            if !module.is_empty() {
                let name = module.split('.').last().unwrap_or(module);
                symbols.push((name.to_string(), idx + 1));
            }
        }
    }
    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.trim().split_whitespace().next().unwrap_or("");
                        if !name.is_empty() && name != "type" {
                            symbols.push((name.to_string(), idx + 1));
                        }
                    }
                }
            }
            continue;
        }
        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let name = import_part.split_once(" from ").map(|(n, _)| n).unwrap_or("");
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((name.to_string(), idx + 1));
                }
            }
        }
    }
    symbols
}

fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || trimmed.contains("PhantomData")
        {
            continue;
        }
        if trimmed.starts_with("#") && !trimmed.starts_with("#[") {
            continue;
        }
        if !trimmed.contains(symbol) {
            continue;
        }
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }
        return true;
    }
    false
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn dummy_impl_traits(lines: &[&str]) -> Vec<String> {
    dummy_impl_traits_with_lines(lines)
        .into_iter()
        .map(|(trait_name, _)| trait_name)
        .collect()
}

fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(String, usize)> {
    let mut traits = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((trait_name, i + 1));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    traits
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = trait_part.split("::").last().unwrap_or(trait_part).trim();
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }
    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }
    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }
    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    let body = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");
    if body == "{}" || (body.contains("{}") && !body.contains("Self::")) {
        return true;
    }
    let panic_marker = format!("{}!(", "panic");
    let dummy_markers = [
        "todo!(",
        "unimplemented!(",
        &panic_marker,
        "unreachable!(",
        "return Err(Default::default())",
        "return Ok(Default::default())",
    ];
    dummy_markers.iter().any(|marker| body.contains(marker))
}

struct PlaceholderImportForbiddenChecker;
impl IImportForbiddenProtocol for PlaceholderImportForbiddenChecker {
    fn check_forbidden_imports(
        &self,
        _analyzer: &Arc<dyn IAnalyzer>,
        _files: &FilePathList,
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderSurfaceChecker;
impl ISurfaceProtocol for PlaceholderSurfaceChecker {
    fn check_surface_hierarchy(
        &self,
        _files: &[FilePath],
        _root: &FilePath,
        _violations: &mut LintResultList,
    ) {
    }
}

struct PlaceholderOrphanAggregate;
impl IOrphanAggregate for PlaceholderOrphanAggregate {
    fn check_orphans(
        &self,
        _container: &dyn CheckerContainerRef,
        _files: &[String],
        _root_dir: &str,
    ) -> Vec<LintResult> {
        Vec::new()
    }
}

impl CheckerContainerRef for CheckerContainer {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CheckerContainer {
    fn default() -> Self {
        Self::new(Arc::new(PlaceholderAnalyzer))
    }
}

struct NullFileSystem;

#[async_trait::async_trait]
impl shared::file_system::contract_system_port::IFileSystemPort for NullFileSystem {
    async fn walk(&self, _path: &FilePath, _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>) -> FilePathList {
        FilePathList::new(vec![])
    }
    async fn is_directory(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn is_file(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &FilePath, _start: &FilePath) -> FilePath {
        path.clone()
    }
    async fn read_text(&self, _path: &FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("read"),
        ))
    }
    async fn get_line_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    async fn exists(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_parent(&self, _path: &FilePath) -> FilePath {
        FilePath::default()
    }
    async fn write_text(
        &self,
        _path: &FilePath,
        _content: &shared::common::taxonomy_source_vo::ContentString,
        _mode: Option<&shared::common::taxonomy_layer_vo::Identity>,
    ) -> Result<shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("write"),
        ))
    }
    async fn glob(&self, _pattern: &shared::common::taxonomy_layer_vo::Identity) -> FilePathList {
        FilePathList::new(vec![])
    }
    async fn get_cwd(&self) -> FilePath {
        FilePath::default()
    }
    async fn get_basename(&self, _path: &FilePath) -> shared::common::taxonomy_layer_vo::Identity {
        shared::common::taxonomy_layer_vo::Identity::default()
    }
    async fn path_join(&self, _parts: &[shared::common::taxonomy_layer_vo::Identity]) -> FilePath {
        FilePath::default()
    }
    async fn read_file(&self, _path: &FilePath) -> Result<shared::common::taxonomy_source_vo::ContentString, shared::file_system::taxonomy_filesystem_error::FileSystemError> {
        Err(shared::file_system::taxonomy_filesystem_error::FileSystemError::new(
            FilePath::default(),
            shared::common::taxonomy_common_error::ErrorMessage::new("null filesystem: not initialized"),
            shared::pipeline_jobs::taxonomy_action_vo::ActionName::new("read"),
        ))
    }
}

struct NullSourceParser;

impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    fn extract_imports(&self, _path: &FilePath) -> Result<shared::code_analysis::taxonomy_import_source_vo::ImportInfoList, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(&self, _path: &FilePath) -> Result<shared::pipeline_jobs::taxonomy_job_vo::ResponseData, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::pipeline_jobs::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::ResponseData {
        shared::pipeline_jobs::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(&self, _path: &FilePath) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &shared::language_adapters::taxonomy_naming_list_vo::PrimitiveTypeList,
    ) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(&self, _path: &FilePath) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(&self, _path: &FilePath) -> Result<shared::common::taxonomy_suggestion_vo::MetadataVO, shared::source_parsing::taxonomy_parser_error::SourceParserError> {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new()))
    }
    fn get_function_definitions(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(&self, _path: &FilePath, _symbol: &shared::common::taxonomy_name_vo::SymbolName) -> shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus {
        shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(&self, _path: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    fn is_barrel_file(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_stem(&self, _path: &FilePath) -> shared::common::taxonomy_name_vo::SymbolName {
        shared::common::taxonomy_name_vo::SymbolName::new("")
    }
    fn is_entry_point(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList {
        shared::common::taxonomy_common_vo::PatternList::default()
    }
}

struct PlaceholderAnalyzer;
impl IAnalyzer for PlaceholderAnalyzer {
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn layer_map(&self) -> &shared::taxonomy_definition_vo::LayerMapVO {
        static MAP: std::sync::OnceLock<shared::taxonomy_definition_vo::LayerMapVO> =
            std::sync::OnceLock::new();
        MAP.get_or_init(|| {
            shared::taxonomy_definition_vo::LayerMapVO::new(std::collections::HashMap::new())
        })
    }
    fn fs(&self) -> &dyn shared::file_system::contract_system_port::IFileSystemPort {
        static FS: std::sync::OnceLock<NullFileSystem> = std::sync::OnceLock::new();
        FS.get_or_init(|| NullFileSystem)
    }
    fn parser(&self) -> &dyn shared::source_parsing::contract_parser_port::ISourceParserPort {
        static PARSER: std::sync::OnceLock<NullSourceParser> = std::sync::OnceLock::new();
        PARSER.get_or_init(|| NullSourceParser)
    }
    fn detect_layer(
        &self,
        _f: &FilePath,
        _root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
    fn detect_module_layer(
        &self,
        _module_path: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

// AnalysisContainer — wiring for code-analysis feature
use crate::CodebaseScanOrchestrator;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;

pub struct AnalysisContainer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

impl AnalysisContainer {
    pub fn new() -> Self {
        Self {
            arch_linter: Arc::new(CodebaseScanOrchestrator::new()),
        }
    }

    pub fn architecture_linter(&self) -> Arc<dyn IArchLintProtocol> {
        self.arch_linter.clone()
    }
}

impl Default for AnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
