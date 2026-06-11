// PURPOSE: MandatoryInheritanceChecker — IMandatoryInheritanceProtocol for AES014: enforce contract implementation (bidirectional)
use crate::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

static TRAIT_RUST_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:pub\s+)?trait\s+([A-Z][A-Za-z0-9_]+)").expect("TRAIT_RUST_RE compile failed")
});
static CLASS_PY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"class\s+([A-Z][A-Za-z0-9_]+)").expect("CLASS_PY_RE compile failed")
});
static INTERFACE_TS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:export\s+)?interface\s+([A-Z][A-Za-z0-9_]+)")
        .expect("INTERFACE_TS_RE compile failed")
});

pub struct MandatoryInheritanceChecker {}

impl Default for MandatoryInheritanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryInheritanceChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMandatoryInheritanceProtocol for MandatoryInheritanceChecker {
    /// One-way: file that imports contract must implement it.
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // Get contract layer's allowed suffixes from config
        let contract_suffixes: Vec<String> = config
            .layers
            .get(&LayerNameVO::new("contract"))
            .map(|d| {
                d.allowed_suffix
                    .values()
                    .iter()
                    .map(|s| format!("_{}", s))
                    .collect()
            })
            .unwrap_or_default();
        if contract_suffixes.is_empty() {
            return;
        }

        // Get current layer's allowed suffixes (implementer suffixes) from config
        let layer_def = match config.layers.get(&LayerNameVO::new(layer)) {
            Some(def) => def,
            None => return,
        };
        let implementer_suffixes = layer_def.allowed_suffix.values();
        if implementer_suffixes.is_empty() {
            return;
        }

        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = filename.rsplit('.').next_back().unwrap_or(filename);
        let own_suffix = stem.rsplit('_').next().unwrap_or("");
        let is_implementer = implementer_suffixes.iter().any(|s| s == own_suffix);
        if !is_implementer {
            return;
        }

        let mut imported: Vec<String> = Vec::new();

        // Rust: `use ...::ContractName;`
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ") && contract_suffixes.iter().any(|s| t.contains(s.as_str())) {
                if let Some(name) = t.split("::").last() {
                    let c = name.trim_end_matches(';').trim();
                    if c.starts_with('I') || c.ends_with("Protocol") || c.ends_with("Port") {
                        imported.push(c.to_string());
                    }
                }
            }
            // Python
            if t.starts_with("from ")
                && contract_suffixes.iter().any(|s| t.contains(s.as_str()))
                && t.contains("import ")
            {
                if let Some(import_part) = t.split("import ").nth(1) {
                    for name in import_part.split(',') {
                        let c = name.trim().split(" as ").next().unwrap_or("").trim();
                        if !c.is_empty()
                            && (c.starts_with('I')
                                || c.ends_with("Protocol")
                                || c.ends_with("Port"))
                        {
                            imported.push(c.to_string());
                        }
                    }
                }
            }
            // JS/TS
            if t.starts_with("import ")
                && t.contains("from")
                && contract_suffixes.iter().any(|s| t.contains(s.as_str()))
            {
                if let Some(brace_start) = t.find('{') {
                    if let Some(brace_end) = t.find('}') {
                        let names_part = &t[brace_start + 1..brace_end];
                        for name in names_part.split(',') {
                            let c = name.trim().split(" as ").next().unwrap_or("").trim();
                            if !c.is_empty()
                                && (c.starts_with('I')
                                    || c.ends_with("Protocol")
                                    || c.ends_with("Port"))
                            {
                                imported.push(c.to_string());
                            }
                        }
                    }
                }
                if !t.contains('{') {
                    let after_import = t.trim_start_matches("import ").trim();
                    if let Some(space_pos) = after_import.find(' ') {
                        let c = after_import[..space_pos].trim();
                        if !c.is_empty()
                            && (c.starts_with('I')
                                || c.ends_with("Protocol")
                                || c.ends_with("Port"))
                        {
                            imported.push(c.to_string());
                        }
                    }
                }
            }
        }

        let has_impl = imported
            .iter()
            .any(|t| content.contains(&format!("impl {} for ", t)));
        let has_python_impl = imported.iter().any(|t| {
            let pattern = format!("({})", t);
            content.lines().any(|line| {
                let lt = line.trim();
                lt.starts_with("class ") && lt.contains(&pattern) && lt.ends_with(':')
            })
        });
        let has_js_impl = imported.iter().any(|t| {
            let pattern = format!("extends {} ", t);
            content.lines().any(|line| {
                let lt = line.trim();
                lt.starts_with("class ") && lt.contains(&pattern) && lt.contains('{')
            })
        });

        if !has_impl && !has_python_impl && !has_js_impl {
            let all_are_deps: bool = imported.iter().all(|t| {
                content.contains(&format!("Arc<dyn {}>", t))
                    || content.contains(&format!("Box<dyn {}>", t))
                    || content.contains(&format!("&dyn {}", t))
                    || content.contains(&format!(": {}", t))
                    || content.contains(&format!(": Optional[{}]", t))
            });
            if !all_are_deps {
                for t in &imported {
                    if !content.contains(&format!("Arc<dyn {}>", t))
                        && !content.contains(&format!("Box<dyn {}>", t))
                        && !content.contains(&format!("&dyn {}", t))
                        && !content.contains(&format!(": {}", t))
                        && !content.contains(&format!(": Optional[{}]", t))
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            0,
                            "AES014",
                            Severity::HIGH,
                            aes014_mandatory_inheritance(t),
                        ));
                    }
                }
            }
        }

        // Flag empty inheritance bodies
        for t in &imported {
            let pattern_py = format!("({})", t);
            let lines: Vec<&str> = content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                let lt = line.trim();
                if lt.starts_with("class ") && lt.contains(&pattern_py) && lt.ends_with(':') {
                    let class_indent = line.len() - line.trim_start().len();
                    let mut body_lines: Vec<&str> = Vec::new();
                    for next_line in lines.iter().skip(idx + 1) {
                        if next_line.trim().is_empty() {
                            continue;
                        }
                        let next_indent = next_line.len() - next_line.trim_start().len();
                        if next_indent <= class_indent {
                            break;
                        }
                        body_lines.push(next_line.trim());
                    }
                    if body_lines.is_empty() || (body_lines.len() == 1 && body_lines[0] == "pass") {
                        violations.push(LintResult::new_arch(
                            file,
                            idx + 1,
                            "AES014",
                            Severity::HIGH,
                            aes014_mandatory_inheritance(t),
                        ));
                    }
                }
                let pattern_js = format!("extends {} ", t);
                if lt.starts_with("class ") && lt.contains(&pattern_js) && lt.contains('{') {
                    if let Some(brace_pos) = lt.find('{') {
                        let after_brace = lt[brace_pos + 1..].trim();
                        if after_brace == "}" {
                            violations.push(LintResult::new_arch(
                                file,
                                idx + 1,
                                "AES014",
                                Severity::HIGH,
                                aes014_mandatory_inheritance(t),
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Bidirectional: contract file must be implemented by expected layer.
    fn check_contract_implementation(
        &self,
        file: &str,
        content: &str,
        all_files: &[String],
        violations: &mut Vec<LintResult>,
    ) {
        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = filename.rsplit('.').next_back().unwrap_or(filename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

        let target_prefix = match suffix {
            "port" => "infrastructure",
            "protocol" => "capabilities",
            "aggregate" => "agent",
            _ => return,
        };

        let trait_name = match extract_trait_name(content) {
            Some(t) => t,
            None => return,
        };

        let mut has_impl = false;
        for cf in all_files {
            let cb = cf.split('/').next_back().unwrap_or("");
            if !cb.starts_with(target_prefix) {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&format!("impl {} for", trait_name)) {
                    has_impl = true;
                    break;
                }
                if c.contains(&format!("({})", trait_name)) {
                    has_impl = true;
                    break;
                }
                if c.contains(&format!("extends {} ", trait_name))
                    || c.contains(&format!("implements {} ", trait_name))
                {
                    has_impl = true;
                    break;
                }
            }
        }

        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES014",
                Severity::HIGH,
                aes014_bidirectional(&trait_name, suffix, target_prefix),
            ));
        }
    }
}

fn extract_trait_name(content: &str) -> Option<String> {
    for cap in TRAIT_RUST_RE.captures_iter(content) {
        let name = cap[1].to_string();
        if name == "For" || name == "And" || name == "Or" || name == "Self" {
            continue;
        }
        return Some(name);
    }
    if let Some(caps) = CLASS_PY_RE.captures(content) {
        return Some(caps[1].to_string());
    }
    if let Some(caps) = INTERFACE_TS_RE.captures(content) {
        return Some(caps[1].to_string());
    }
    None
}

fn aes014_mandatory_inheritance(contracts: &str) -> String {
    format!("AES014 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}

fn aes014_bidirectional(trait_name: &str, contract_suffix: &str, expected_layer: &str) -> String {
    format!(
        "AES014 MANDATORY_INHERITANCE: Contract {} '{}' must be implemented by {}_* layer.\n        WHY? Contracts define interfaces that must be fulfilled by the expected layer.\n        FIX: Create a {}_* file that implements '{}'.",
        contract_suffix, trait_name, expected_layer, expected_layer, trait_name
    )
}
