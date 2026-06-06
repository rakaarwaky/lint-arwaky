use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::capabilities::architecture_compliance_analyzer::ArchComplianceAnalyzer;
use crate::contract::source_parser_port::ISourceParserPort;
use crate::contract::source_system_port::IFileSystemPort;
use crate::taxonomy::{default_aes_config, ArchitectureConfig, LintResult, LintResultList};

pub fn collect_rs_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if !dir.exists() || !dir.is_dir() {
        return files;
    }
    collect_rs_files_recursive(dir, &mut files);
    files
}

fn collect_rs_files_recursive(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if dir_name == "target" || dir_name == ".git" || dir_name == ".opencode" {
                    continue;
                }
                collect_rs_files_recursive(&path, files);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }
}

/// Walk up from `start` looking for the YAML config file.
/// Returns the parsed ArchitectureConfig if found, or None.
fn try_load_yaml_config(start: &Path) -> Option<ArchitectureConfig> {
    const CONFIG_NAMES: &[&str] = &["lint_arwaky.config.rust.yaml", "lint_arwaky.config.yaml"];
    let mut dir = start;
    loop {
        for name in CONFIG_NAMES {
            let candidate = dir.join(name);
            if candidate.is_file() {
                if let Ok(content) = fs::read_to_string(&candidate) {
                    if let Ok(raw) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        if let Some(arch_val) = raw.get("architecture") {
                            let json = serde_json::to_value(arch_val).ok()?;
                            if let Ok(cfg) = serde_json::from_value::<ArchitectureConfig>(json) {
                                return Some(cfg);
                            }
                        }
                    }
                }
            }
        }
        match dir.parent() {
            Some(p) if p != dir => dir = p,
            _ => break,
        }
    }
    None
}

pub fn load_config(project_root: Option<&Path>, src_dir: &Path) -> ArchitectureConfig {
    let search_start = project_root.unwrap_or_else(|| src_dir.parent().unwrap_or(src_dir));
    try_load_yaml_config(search_start).unwrap_or_else(default_aes_config)
}

pub fn run_lint_with_deps(
    src_dir: &Path,
    project_root: Option<&Path>,
    fs: Arc<dyn IFileSystemPort>,
    parser: Arc<dyn ISourceParserPort>,
) -> Vec<LintResult> {
    let config = load_config(project_root, src_dir);
    let analyzer = ArchComplianceAnalyzer::new(config);
    let files = collect_rs_files(src_dir);
    if files.is_empty() {
        return Vec::new();
    }
    let root_dir = src_dir.to_string_lossy().to_string();
    analyzer.execute(&files, &root_dir)
}

pub fn format_report(results: &[LintResult], project_root: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("=".repeat(60));
    lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
    lines.push("=".repeat(60));
    lines.push(format!("  Project: {}", project_root));
    lines.push(format!("  Files scanned: {}", results.len()));
    lines.push("=".repeat(60));
    lines.push("".to_string());

    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();

    for r in results {
        match r.severity {
            crate::taxonomy::Severity::CRITICAL => critical.push(r),
            crate::taxonomy::Severity::HIGH => high.push(r),
            crate::taxonomy::Severity::MEDIUM => medium.push(r),
            crate::taxonomy::Severity::LOW => low.push(r),
            _ => medium.push(r),
        }
    }

    if !critical.is_empty() {
        lines.push(format!("  [CRITICAL] {} violations", critical.len()));
        lines.push("-".repeat(60));
        for r in &critical {
            lines.push(format!(
                "  {}:{} [{}] {}",
                r.file.value, r.line.value, format!("{}", r.code), r.message.value
            ));
        }
        lines.push("".to_string());
    }

    if !high.is_empty() {
        lines.push(format!("  [HIGH] {} violations", high.len()));
        lines.push("-".repeat(60));
        for r in &high {
            lines.push(format!(
                "  {}:{} [{}] {}",
                r.file.value, r.line.value, format!("{}", r.code), r.message.value
            ));
        }
        lines.push("".to_string());
    }

    if !medium.is_empty() {
        lines.push(format!("  [MEDIUM] {} violations", medium.len()));
        lines.push("-".repeat(60));
        for r in &medium {
            lines.push(format!(
                "  {}:{} [{}] {}",
                r.file.value, r.line.value, format!("{}", r.code), r.message.value
            ));
        }
        lines.push("".to_string());
    }

    if !low.is_empty() {
        lines.push(format!("  [LOW] {} violations", low.len()));
        lines.push("-".repeat(60));
        for r in &low {
            lines.push(format!(
                "  {}:{} [{}] {}",
                r.file.value, r.line.value, format!("{}", r.code), r.message.value
            ));
        }
        lines.push("".to_string());
    }

    let total = results.len();
    lines.push("=".repeat(60));
    lines.push(format!("  Total violations: {}", total));
    if total == 0 {
        lines.push("  Status: PASS - No AES violations detected".to_string());
    } else {
        lines.push("  Status: FAIL - AES violations detected".to_string());
    }
    lines.push("=".repeat(60));
    lines.join("\n")
}


pub struct ArchLintHandler {
    fs: Arc<dyn IFileSystemPort>,
    parser: Arc<dyn ISourceParserPort>,
}

impl ArchLintHandler {
    pub fn new(fs: Arc<dyn IFileSystemPort>, parser: Arc<dyn ISourceParserPort>) -> Self {
        Self { fs, parser }
    }
}

impl crate::contract::architecture_lint_protocol::IArchLintProtocol for ArchLintHandler {
    fn run_self_lint(&self, project_root: &str) -> LintResultList {
        let src_dir = Path::new(project_root).join("src-rust");
        let results = run_lint_with_deps(
            &src_dir,
            Some(Path::new(project_root)),
            self.fs.clone(),
            self.parser.clone(),
        );
        LintResultList::new(results)
    }

    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList {
        let results = run_lint_with_deps(
            Path::new(src_dir),
            None,
            self.fs.clone(),
            self.parser.clone(),
        );
        LintResultList::new(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        format_report(&results.values, project_root)
    }
}
