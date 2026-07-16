use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;
use std::sync::OnceLock;

pub struct ContractOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self {
            extractor: Arc::new(
                crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new(),
            ),
        }
    }
}

impl ContractOrphanAnalyzer {
    pub fn new(extractor: Arc<dyn IOrphanFilenameExtractorProtocol>) -> Self {
        Self { extractor }
    }
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        is_contract_orphan(
            f,
            root_dir,
            file_definitions,
            inheritance_map,
            all_files,
            &self.extractor,
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ORCHESTRATOR
// ═══════════════════════════════════════════════════════════════════════════════

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[FilePath],
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let trait_name = match extract_contract_trait_name(&content) {
        Some(t) => t,
        None => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let search_files = build_search_files(all_files, root_dir);

    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    // ── Check 1: trait not implemented by expected layer ──
    if !check_implemented(&search_files, &trait_name, target_prefix, extractor) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not implemented by any {} file.",
                suffix, trait_name, target_prefix
            ),
        );
    }

    // ── Check 2: port/protocol not called by orchestrator/container/capabilities/surface ──
    if suffix == "port" || suffix == "protocol" {
        if !check_called_port_protocol(&search_files, &trait_name, extractor) {
            return orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract {} '{}' not called by any orchestrator, container, capabilities, or surface file.",
                    suffix, trait_name
                ),
            );
        }
    }

    // ── Check 3: aggregate not called by surface/container ──
    if suffix == "aggregate" {
        if !check_called_aggregate(&search_files, &trait_name, extractor) {
            return orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract aggregate '{}' not called by any surface or container file.",
                    trait_name
                ),
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULAR DETECTION: IMPLEMENT / CALL / WIRE
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if trait is implemented by any file in the target layer.
fn check_implemented(
    search_files: &[String],
    trait_name: &str,
    target_prefix: &str,
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> bool {
    for cf in search_files {
        let cb = extractor
            .file_basename(&FilePath { value: cf.clone() })
            .value;
        let is_target_layer = cb.starts_with(target_prefix);
        let is_container_impl = cb.starts_with("root_") && cb.ends_with("_container.rs");
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if has_rust_impl(&c, trait_name)
                || has_py_impl(&c, trait_name)
                || has_ts_impl(&c, trait_name)
            {
                return true;
            }
        }
    }
    false
}

/// Check if port/protocol is called by orchestrator/container/capabilities/surface.
fn check_called_port_protocol(
    search_files: &[String],
    trait_name: &str,
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> bool {
    for cf in search_files {
        let cb = extractor
            .file_basename(&FilePath { value: cf.clone() })
            .value;
        let is_orchestrator = cb.starts_with("agent_")
            && (cb.ends_with("_orchestrator.rs")
                || cb.ends_with("_orchestrator.py")
                || cb.ends_with("_orchestrator.ts")
                || cb.ends_with("_orchestrator.js"));
        let is_container = cb.ends_with("_container.rs")
            || cb.ends_with("_container.py")
            || cb.ends_with("_container.ts")
            || cb.ends_with("_container.js");
        let is_capabilities = cb.starts_with("capabilities_");
        let is_surface = cb.starts_with("surface_");

        if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if has_rust_call(&c, trait_name)
                || has_py_call(&c, trait_name)
                || has_ts_call(&c, trait_name)
                || has_rust_wire(&c, trait_name)
                || has_py_wire(&c, trait_name)
                || has_ts_wire(&c, trait_name)
            {
                return true;
            }
        }
    }
    false
}

/// Check if aggregate is called by surface/container.
fn check_called_aggregate(
    search_files: &[String],
    trait_name: &str,
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> bool {
    for cf in search_files {
        let cb = extractor
            .file_basename(&FilePath { value: cf.clone() })
            .value;
        let is_surface = cb.starts_with("surface_");
        let is_container = cb.ends_with("_container.rs")
            || cb.ends_with("_container.py")
            || cb.ends_with("_container.ts")
            || cb.ends_with("_container.js");

        if !is_surface && !is_container {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if has_rust_call(&c, trait_name)
                || has_py_call(&c, trait_name)
                || has_ts_call(&c, trait_name)
                || has_rust_wire(&c, trait_name)
                || has_py_wire(&c, trait_name)
                || has_ts_wire(&c, trait_name)
            {
                return true;
            }
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// RUST DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Rust: `impl Trait for Type { ... }`
fn has_rust_impl(content: &str, trait_name: &str) -> bool {
    content.contains(&format!("impl {} for", trait_name))
        || content.lines().any(|ln| {
            let t = ln.trim();
            t.starts_with("impl") && t.contains(trait_name) && t.contains(" for")
        })
}

/// Rust: `Type::method()` or `instance.method()` where method is from trait
fn has_rust_call(content: &str, trait_name: &str) -> bool {
    // Direct usage: `use ...::TraitName` or `TraitName::method()`
    content.contains(&format!("use {}", trait_name))
        || content.contains(&format!("{}::", trait_name))
        || content.contains(&format!("<dyn {}>", trait_name))
        || content.contains(&format!("Arc<dyn {}>", trait_name))
}

/// Rust: `Arc::new(Type::new())` or `Type::new()` in constructor context
fn has_rust_wire(content: &str, trait_name: &str) -> bool {
    content.contains(&format!("Arc::new({}", trait_name))
        || content.contains(&format!("Box::new({}", trait_name))
        || content.contains(&format!("{}::new(", trait_name))
}

// ═══════════════════════════════════════════════════════════════════════════════
// PYTHON DETECTION (abc abstractmethod)
// ═══════════════════════════════════════════════════════════════════════════════

/// Python: `class Type(ABC):` with `@abstractmethod` — abstract base class
fn has_py_impl(content: &str, trait_name: &str) -> bool {
    // Pattern 1: class Type(BaseTrait): — inheritance
    let inherit_pattern = format!(
        r"class\s+\w+\([^)]*\b{}\b[^)]*\)",
        regex::escape(trait_name)
    );
    if regex_match(&inherit_pattern, content) {
        return true;
    }

    // Pattern 2: class Type(ABC): with @abstractmethod — abstract base class
    if content.contains("from abc import")
        && content.contains("ABC")
        && content.contains("@abstractmethod")
    {
        // Check if this file defines the abstract class itself
        let class_pattern = format!(r"class\s+{}\s*\(", regex::escape(trait_name));
        if regex_match(&class_pattern, content) {
            return true;
        }
    }

    false
}

/// Python: `instance.method()` or `Type.method()` where method is from trait
fn has_py_call(content: &str, trait_name: &str) -> bool {
    // Direct import: `from module import TraitName`
    content.contains(&format!("import {}", trait_name))
        // Usage: `instance.method()` — we check if trait_name appears as a type hint or call
        || content.contains(&format!("{}.", trait_name))
        // Type hint: `def func(x: TraitName)`
        || content.contains(&format!(": {}", trait_name))
        || content.contains(&format!(": {}", trait_name))
}

/// Python: `TraitName()` or `TraitName(dep1, dep2)` in constructor
fn has_py_wire(content: &str, trait_name: &str) -> bool {
    // Constructor injection: `self.dep = TraitName(dep1, dep2)`
    content.contains(&format!("{}(", trait_name))
        // Type annotation in constructor: `def __init__(self, dep: TraitName)`
        || content.contains(&format!("{}:", trait_name))
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPESCRIPT DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// TypeScript: `class Type implements TraitName` or `class Type extends TraitName`
fn has_ts_impl(content: &str, trait_name: &str) -> bool {
    content.contains(&format!("implements {}", trait_name))
        || content.contains(&format!("extends {}", trait_name))
}

/// TypeScript: `instance.method()` or `Type.method()` where method is from trait
fn has_ts_call(content: &str, trait_name: &str) -> bool {
    // Import: `import { TraitName } from ...`
    content.contains(&format!("import {{ {} }}", trait_name))
        || content.contains(&format!("import {{ {} }}", trait_name))
        // Usage: `Type.method()` or `<Type>`
        || content.contains(&format!("{}.", trait_name))
        || content.contains(&format!("<{}>", trait_name))
        || content.contains(&format!(": {}", trait_name))
}

/// TypeScript: `new TraitName()` or `TraitName(dep1, dep2)` in constructor
fn has_ts_wire(content: &str, trait_name: &str) -> bool {
    // Constructor injection: `new TraitName(dep1, dep2)`
    content.contains(&format!("new {}(", trait_name))
        // Type annotation: `dep: TraitName`
        || content.contains(&format!(": {}", trait_name))
        // Generic: `Arc<dyn TraitName>`
        || content.contains(&format!("Arc<dyn {}>", trait_name))
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn regex_match(pattern: &str, content: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(content))
        .unwrap_or(false)
}

fn build_search_files(all_files: &[FilePath], root_dir: &FilePath) -> Vec<String> {
    let mut search_files: Vec<String> = all_files.iter().map(|fp| fp.value().to_string()).collect();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_source_files(&ws_path, &mut search_files);
        }
    }
    search_files
}

fn orphan_result(
    suffix: &str,
    trait_name: &str,
    target_prefix: &str,
    reason: &str,
) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::ContractOrphan {
            suffix: suffix.to_string(),
            trait_name: trait_name.to_string(),
            target_layer: target_prefix.to_string(),
            reason: Some(reason.to_string().into()),
        }
        .to_string(),
        Severity::LOW,
    )
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAIT NAME EXTRACTION
// ═══════════════════════════════════════════════════════════════════════════════

fn re_contract_rust() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_contract_py() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_ts_interface_export() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_interface() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn extract_contract_trait_name(content: &str) -> Option<String> {
    let code_lines: String = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*")
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(re) = re_contract_rust() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    if let Some(re) = re_ts_interface_export() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    if let Some(re) = re_interface() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    re_contract_py()
        .and_then(|re| re.captures(&code_lines))
        .map(|caps| caps[1].to_string())
}

fn collect_source_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                collect_source_files(&path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    if let Some(s) = path.to_str() {
                        files.push(s.to_string());
                    }
                }
            }
        }
    }
}
