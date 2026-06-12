// PURPOSE: ICycleAnalysisProtocol + DefaultCycleAnalysisProtocol — port trait and default impl for circular dependency detection (AES015)
use std::collections::{HashMap, HashSet};

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleAnalysisProtocol: Send + Sync {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub struct DefaultCycleAnalysisProtocol {}

fn try_resolve_candidates(base_path: &str, module_path: &str, file_set: &HashSet<String>) -> Option<String> {
    let exts = ["rs", "py", "ts", "js"];
    for ext in &exts {
        let candidate = format!("{}/{}.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/mod.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/__init__.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn resolve_import_to_file(
    module: &str,
    source_file: &FilePath,
    root_dir: &FilePath,
    file_set: &HashSet<String>,
) -> Option<String> {
    let source_dir = std::path::Path::new(source_file.value())
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    if module.starts_with("crate::") {
        let relative = module.trim_start_matches("crate::");
        let segments: Vec<&str> = relative.split("::").collect();
        let module_path = segments.join("/");
        return try_resolve_candidates(root_dir.value(), &module_path, file_set);
    }

    if module.starts_with("self::") {
        let relative = module.trim_start_matches("self::");
        let segments: Vec<&str> = relative.split("::").collect();
        let module_path = segments.join("/");
        return try_resolve_candidates(&source_dir, &module_path, file_set);
    }

    if module.starts_with("super::") {
        let mut current_dir = std::path::PathBuf::from(&source_dir);
        let mut remaining = module;
        while remaining.starts_with("super::") {
            remaining = remaining.trim_start_matches("super::");
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            }
        }
        let segments: Vec<&str> = remaining.split("::").filter(|s| !s.is_empty()).collect();
        let module_path = segments.join("/");
        if !module_path.is_empty() {
            return try_resolve_candidates(&current_dir.to_string_lossy(), &module_path, file_set);
        }
        return None;
    }

    if module.starts_with('.') {
        let base = std::path::Path::new(&source_dir);
        let resolved = base.join(module.trim_start_matches('.').trim_start_matches('/'));
        let resolved_str = resolved.to_string_lossy().to_string();

        let exts = ["rs", "py", "ts", "js"];
        for ext in &exts {
            let candidate = format!("{}.{}", resolved_str, ext);
            if file_set.contains(&candidate) {
                return Some(candidate);
            }
        }
        for ext in &exts {
            let candidate = format!("{}/mod.{}", resolved_str, ext);
            if file_set.contains(&candidate) {
                return Some(candidate);
            }
        }
        for ext in &exts {
            let candidate = format!("{}/__init__.{}", resolved_str, ext);
            if file_set.contains(&candidate) {
                return Some(candidate);
            }
        }
        return None;
    }

    let segments: Vec<&str> = module.split('.').collect();
    let module_path = segments.join("/");
    try_resolve_candidates(root_dir.value(), &module_path, file_set)
}

fn find_cycle_dfs(
    node: &str,
    adjacency: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    if in_stack.contains(node) {
        let cycle_start = path.iter().position(|n| n == node);
        if let Some(start) = cycle_start {
            let mut cycle = path[start..].to_vec();
            cycle.push(node.to_string());
            return Some(cycle);
        }
    }
    if visited.contains(node) {
        return None;
    }

    visited.insert(node.to_string());
    in_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if let Some(cycle) = find_cycle_dfs(neighbor, adjacency, visited, in_stack, path) {
                return Some(cycle);
            }
        }
    }

    path.pop();
    in_stack.remove(node);
    None
}

#[async_trait]
impl ICycleAnalysisProtocol for DefaultCycleAnalysisProtocol {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_set: HashSet<String> = files.values.iter().map(|f| f.value().to_string()).collect();

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files.values {
            let imports = match analyzer.parser().extract_imports(file) {
                Ok(imp) => imp,
                Err(_) => continue,
            };

            for imp in imports.values {
                if let Some(target) =
                    resolve_import_to_file(&imp.module, file, root_dir, &file_set)
                {
                    adjacency
                        .entry(file.value().to_string())
                        .or_default()
                        .push(target);
                }
            }
        }

        let mut global_visited: HashSet<String> = HashSet::new();

        for file in &files.values {
            let file_str = file.value().to_string();
            if global_visited.contains(&file_str) {
                continue;
            }

            let mut visited: HashSet<String> = HashSet::new();
            let mut in_stack: HashSet<String> = HashSet::new();
            let mut path: Vec<String> = Vec::new();

            if let Some(cycle) =
                find_cycle_dfs(&file_str, &adjacency, &mut visited, &mut in_stack, &mut path)
            {
                let cycle_display = cycle.join(" -> ");
                results.push(LintResult {
                    file: file.clone(),
                    line: crate::common::taxonomy_common_vo::LineNumber::new(1),
                    column: crate::common::taxonomy_common_vo::ColumnNumber::new(0),
                    code: ErrorCode::raw("AES015"),
                    message: LintMessage::new(format!(
                        "Circular dependency detected: {}",
                        cycle_display
                    )),
                    source: Some(AdapterName::raw("architecture")),
                    severity: Severity::CRITICAL,
                    enclosing_scope: Some(ScopeRef {
                        name: DescriptionVO::new(String::new()),
                        kind: DescriptionVO::new(String::new()),
                        file: None,
                        start_line: None,
                        end_line: None,
                    }),
                    related_locations: crate::common::taxonomy_lint_vo::LocationList::new(),
                });
            }

            global_visited.extend(visited);
        }
    }
}
