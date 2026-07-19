use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IContractOrphanProtocol, IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ContractOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        let search_files = self.build_search_files(all_files, root_dir);
        let mut contents: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut basenames: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for path in &search_files {
            if contents.contains_key(path) {
                continue;
            }
            let fp = FilePath {
                value: path.clone(),
            };
            let content = self.cache.read_cached(&fp).value;
            if !content.is_empty() {
                contents.insert(path.clone(), content.clone());
                let basename = self.extractor.file_basename(&fp).value;
                basenames.insert(path.clone(), basename);
            }
        }

        Self::is_contract_orphan_util(f, &contents, &basenames, self.extractor.as_ref())
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl ContractOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }

    fn build_search_files(&self, all_files: &[FilePath], root_dir: &FilePath) -> Vec<String> {
        let mut search_files: Vec<String> =
            all_files.iter().map(|fp| fp.value().to_string()).collect();
        let root_path = std::path::Path::new(root_dir.value());
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if ws_path.exists() {
                self.collect_source_files(&ws_path, &mut search_files);
            }
        }
        search_files
    }

    fn collect_source_files(&self, dir: &std::path::Path, files: &mut Vec<String>) {
        let dir_str = dir.to_str().unwrap_or("");
        let dir_fp = shared::common::taxonomy_path_vo::FilePath::new(dir_str).unwrap_or_default();
        if self.cache.is_symlink(&dir_fp).value() {
            return;
        }

        let entries = self.cache.read_dir(&dir_fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                self.collect_source_files(path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    files.push(entry_path.value().to_string());
                }
            }
        }
    }

    // ── taxonomy_contract_regex_utility ──

    pub fn re_contract_rust() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    pub fn re_contract_py() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:class\s+([A-Za-z0-9_]+)\s*\([^)]*ABC[^)]*\)|class\s+([A-Za-z0-9_]+)\s*\([^)]*Protocol[^)]*\))").ok()).as_ref()
    }

    pub fn re_contract_py_fallback() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)\s*[\(:]").ok())
            .as_ref()
    }

    pub fn re_ts_interface_export() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    pub fn re_interface() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    pub fn word_boundary_re(trait_name: &str) -> Regex {
        let pattern = format!(r"\b{}\b", regex::escape(trait_name));
        match Regex::new(&pattern) {
            Ok(re) => re,
            Err(_) => Self::never_match_regex(),
        }
    }

    fn never_match_regex() -> Regex {
        match Regex::new("") {
            Ok(re) => re,
            Err(_) => std::process::abort(),
        }
    }

    // ── taxonomy_contract_detection_utility ──

    pub fn has_rust_impl(content: &str, rust_impl_pattern: &str, re_trait: &Regex) -> bool {
        content.contains(rust_impl_pattern)
            || content.lines().any(|ln| {
                let t = ln.trim();
                t.starts_with("impl") && re_trait.is_match(t) && t.contains(" for")
            })
    }

    pub fn has_rust_call(content: &str, re_trait: &Regex) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
                continue;
            }
            if (t.starts_with("use ")
                || t.contains("::")
                || t.contains("<dyn ")
                || t.contains("Arc<dyn "))
                && re_trait.is_match(t)
            {
                return true;
            }
        }
        false
    }

    pub fn has_rust_wire(content: &str, re_trait: &Regex) -> bool {
        let lines: Vec<&str> = content.lines().collect();
        let total = lines.len();
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
                continue;
            }
            if !re_trait.is_match(t) {
                continue;
            }

            let end = std::cmp::min(i + 30, total);
            if lines[i..end].iter().any(|&ln| {
                let tl = ln.trim();
                !tl.starts_with("//")
                    && !tl.starts_with("/*")
                    && !tl.starts_with('*')
                    && (tl.contains("Arc::new(") || tl.contains("Box::new("))
            }) {
                return true;
            }

            let is_impl_line = t.contains("impl") && t.contains("for") && re_trait.is_match(t);
            if !is_impl_line {
                let has_impl = lines.iter().any(|&ln| {
                    let lt = ln.trim();
                    !lt.starts_with("//")
                        && !lt.starts_with("/*")
                        && re_trait.is_match(lt)
                        && lt.contains("impl")
                        && lt.contains("for")
                });
                if has_impl {
                    let has_new = lines.iter().any(|&ln| {
                        let tl = ln.trim();
                        !tl.starts_with("//")
                            && !tl.starts_with("/*")
                            && (tl.contains("Arc::new(") || tl.contains("Box::new("))
                    });
                    if has_new {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn has_py_impl(content: &str, trait_name: &str) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                continue;
            }
            if t.starts_with("class ") && t.contains(trait_name) && t.contains('(') {
                return true;
            }
        }
        false
    }

    pub fn has_py_call(content: &str, re_trait: &Regex) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                continue;
            }
            if (t.starts_with("from ") || t.starts_with("import ")) && re_trait.is_match(t) {
                return true;
            }
            if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
                return true;
            }
        }
        false
    }

    pub fn has_py_wire(content: &str, re_trait: &Regex) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                continue;
            }
            if re_trait.is_match(t) && t.contains('(') && !t.starts_with("class ") {
                return true;
            }
        }
        false
    }

    pub fn has_ts_impl(content: &str, trait_name: &str) -> bool {
        let re = Self::word_boundary_re(trait_name);
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("/*") {
                continue;
            }
            if (t.contains("implements ") || t.contains("extends ")) && re.is_match(t) {
                return true;
            }
        }
        false
    }

    pub fn has_ts_call(content: &str, re_trait: &Regex) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("/*") {
                continue;
            }
            if t.starts_with("import ") && re_trait.is_match(t) {
                return true;
            }
            if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
                return true;
            }
        }
        false
    }

    pub fn has_ts_wire(content: &str, re_trait: &Regex) -> bool {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("/*") {
                continue;
            }
            if re_trait.is_match(t) && t.contains("new ") && t.contains('(') {
                return true;
            }
        }
        false
    }

    pub fn strip_comments(content: &str, ext: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_block_comment = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if in_block_comment {
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            if ext == "rs" {
                if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                    if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                        in_block_comment = true;
                    }
                    continue;
                }
                let code_line = if let Some(pos) = line.find("//") {
                    &line[..pos]
                } else {
                    line
                };
                result.push_str(code_line);
                result.push('\n');
                continue;
            }

            if ext == "py" {
                if trimmed.starts_with('#') {
                    continue;
                }
                let code_line = if let Some(pos) = line.find('#') {
                    &line[..pos]
                } else {
                    line
                };
                result.push_str(code_line);
                result.push('\n');
                continue;
            }

            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                    in_block_comment = true;
                }
                continue;
            }
            let code_line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            result.push_str(code_line);
            result.push('\n');
        }

        result
    }

    pub fn extract_contract_trait_name(content: &str, file_path: &str) -> Option<String> {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let code = Self::strip_comments(content, ext);

        match ext {
            "rs" => Self::re_contract_rust()
                .and_then(|re| re.captures(&code))
                .map(|caps| caps[1].to_string()),
            "py" => {
                if let Some(caps) = Self::re_contract_py().and_then(|re| re.captures(&code)) {
                    caps.get(1)
                        .or_else(|| caps.get(2))
                        .map(|m| m.as_str().to_string())
                } else {
                    Self::re_contract_py_fallback()
                        .and_then(|re| re.captures(&code))
                        .map(|caps| caps[1].to_string())
                }
            }
            "ts" | "tsx" | "js" | "jsx" => Self::re_ts_interface_export()
                .and_then(|re| re.captures(&code))
                .or_else(|| Self::re_interface().and_then(|re| re.captures(&code)))
                .map(|caps| caps[1].to_string()),
            _ => None,
        }
    }

    // ── taxonomy_contract_check_utility ──

    const SUFFIX_PORT: &'static str = "port";
    const SUFFIX_PROTOCOL: &'static str = "protocol";
    const SUFFIX_AGGREGATE: &'static str = "aggregate";
    const LAYER_INFRASTRUCTURE: &'static str = "infrastructure";
    const LAYER_CAPABILITIES: &'static str = "capabilities";
    const LAYER_AGENT: &'static str = "agent";

    pub fn check_implemented(
        contents: &HashMap<String, String>,
        basenames: &HashMap<String, String>,
        trait_name: &str,
        target_prefix: &str,
    ) -> bool {
        let rust_impl_pattern = format!("impl {} for", trait_name);
        let re_trait = Self::word_boundary_re(trait_name);

        for (path, content) in contents {
            let bn = match basenames.get(path) {
                Some(b) => b.as_str(),
                None => continue,
            };
            let is_target_layer = bn.starts_with(target_prefix);
            let is_container_impl = bn.starts_with("root_")
                && (bn.ends_with("_container.rs")
                    || bn.ends_with("_container.py")
                    || bn.ends_with("_container.ts")
                    || bn.ends_with("_container.js"));
            let is_taxonomy_impl = bn.starts_with("taxonomy_");
            if !is_target_layer && !is_container_impl && !is_taxonomy_impl {
                continue;
            }
            if Self::has_rust_impl(content, &rust_impl_pattern, &re_trait)
                || Self::has_py_impl(content, trait_name)
                || Self::has_ts_impl(content, trait_name)
            {
                return true;
            }
        }
        false
    }

    pub fn check_called(
        contents: &HashMap<String, String>,
        basenames: &HashMap<String, String>,
        trait_name: &str,
    ) -> bool {
        let re_trait = Self::word_boundary_re(trait_name);

        for (path, content) in contents {
            let bn = match basenames.get(path) {
                Some(b) => b.as_str(),
                None => continue,
            };
            let is_relevant = bn.starts_with("agent_")
                || bn.ends_with("_container.rs")
                || bn.ends_with("_container.py")
                || bn.ends_with("_container.ts")
                || bn.ends_with("_container.js")
                || bn.starts_with("capabilities_")
                || bn.starts_with("surface_");

            if !is_relevant {
                continue;
            }
            if Self::has_rust_call(content, &re_trait)
                || Self::has_py_call(content, &re_trait)
                || Self::has_ts_call(content, &re_trait)
            {
                return true;
            }
        }
        false
    }

    pub fn check_wired(
        contents: &HashMap<String, String>,
        basenames: &HashMap<String, String>,
        trait_name: &str,
    ) -> bool {
        let re_trait = Self::word_boundary_re(trait_name);

        for (path, content) in contents {
            let bn = match basenames.get(path) {
                Some(b) => b.as_str(),
                None => continue,
            };
            let is_relevant = bn.starts_with("agent_")
                || bn.ends_with("_container.rs")
                || bn.ends_with("_container.py")
                || bn.ends_with("_container.ts")
                || bn.ends_with("_container.js")
                || bn.starts_with("capabilities_")
                || bn.starts_with("surface_");

            if !is_relevant {
                continue;
            }
            if Self::has_rust_wire(content, &re_trait)
                || Self::has_py_wire(content, &re_trait)
                || Self::has_ts_wire(content, &re_trait)
            {
                return true;
            }
        }
        false
    }

    pub fn not_orphan() -> OrphanIndicatorResult {
        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }

    pub fn orphan_result(
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

    pub fn is_contract_orphan_util(
        f: &FilePath,
        contents: &HashMap<String, String>,
        basenames: &HashMap<String, String>,
        extractor: &dyn IOrphanFilenameExtractorProtocol,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = extractor.file_suffix(f).value;

        let content = match contents.get(fp) {
            Some(c) => c.as_str(),
            None => return Self::not_orphan(),
        };

        let trait_name = match Self::extract_contract_trait_name(content, fp) {
            Some(t) => t,
            None => return Self::not_orphan(),
        };

        let target_prefix = match suffix.as_str() {
            Self::SUFFIX_PORT => Self::LAYER_INFRASTRUCTURE,
            Self::SUFFIX_PROTOCOL => Self::LAYER_CAPABILITIES,
            Self::SUFFIX_AGGREGATE => Self::LAYER_AGENT,
            _ => return Self::not_orphan(),
        };

        if !Self::check_implemented(contents, basenames, &trait_name, target_prefix) {
            return Self::orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract {} '{}' not implemented by any {} file.",
                    suffix, trait_name, target_prefix
                ),
            );
        }

        if (suffix == Self::SUFFIX_PORT || suffix == Self::SUFFIX_PROTOCOL)
            && !Self::check_called(contents, basenames, &trait_name)
        {
            return Self::orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract {} '{}' not called by any orchestrator, container, capabilities, or surface file.",
                    suffix, trait_name
                ),
            );
        }

        if (suffix == Self::SUFFIX_PORT || suffix == Self::SUFFIX_PROTOCOL)
            && !Self::check_wired(contents, basenames, &trait_name)
        {
            return Self::orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract {} '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                    suffix, trait_name
                ),
            );
        }

        if suffix == Self::SUFFIX_AGGREGATE && !Self::check_called(contents, basenames, &trait_name)
        {
            return Self::orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract aggregate '{}' not called by any surface or container file.",
                    trait_name
                ),
            );
        }

        if suffix == Self::SUFFIX_AGGREGATE && !Self::check_wired(contents, basenames, &trait_name)
        {
            return Self::orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract aggregate '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                    trait_name
                ),
            );
        }

        Self::not_orphan()
    }
}
