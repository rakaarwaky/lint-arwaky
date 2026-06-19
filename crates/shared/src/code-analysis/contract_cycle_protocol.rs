// PURPOSE: ICycleAnalysisProtocol + DefaultCycleAnalysisProtocol — port trait and default impl for circular dependency detection (AES205)
use std::collections::{HashMap, HashSet};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
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

fn find_rust_crate_root(source_file: &str) -> Option<std::path::PathBuf> {
    let mut current = std::path::Path::new(source_file).parent()?;
    while !current.join("Cargo.toml").exists() {
        current = current.parent()?;
    }
    Some(current.join("src"))
}

fn try_resolve_candidates(
    base_path: &str,
    module_path: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
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
    for ext in &exts {
        let candidate = format!("{}/{}/index.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    // Also check bare module path (no base prefix) for flat file sets
    for ext in &exts {
        let candidate = format!("{}.{}", module_path, ext);
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

    // 1. Handle relative imports starting with dots (Python, JS/TS)
    if module.starts_with('.') {
        let mut current_dir = std::path::PathBuf::from(&source_dir);
        let mut remaining = module;

        if remaining.starts_with("./") || remaining.starts_with("../") {
            // JS/TS style
            if let Some(r) = remaining.strip_prefix("./") {
                remaining = r;
            }
            while let Some(r) = remaining.strip_prefix("../") {
                remaining = r;
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                }
            }
            let remaining_path = remaining.replace('\\', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        } else {
            // Python style (count leading dots)
            let mut dots_count = 0;
            while remaining.starts_with('.') {
                dots_count += 1;
                remaining = &remaining[1..];
            }
            if dots_count > 1 {
                for _ in 0..(dots_count - 1) {
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
            }
            let remaining_path = remaining.replace('.', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        }
    }

    // 2. Handle Rust-specific imports
    let is_rust = source_file.value().ends_with(".rs");
    if is_rust {
        if let Some(crate_root) = find_rust_crate_root(source_file.value()) {
            let mut normalized = module.to_string();
            let mut resolved_base = crate_root.clone();

            if normalized.starts_with("crate::") {
                normalized = normalized.trim_start_matches("crate::").to_string();
            } else if normalized.starts_with("self::") {
                normalized = normalized.trim_start_matches("self::").to_string();
                resolved_base = std::path::PathBuf::from(&source_dir);
            } else if normalized.starts_with("super::") {
                let mut current_dir = std::path::PathBuf::from(&source_dir);
                while normalized.starts_with("super::") {
                    normalized = normalized.trim_start_matches("super::").to_string();
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
                resolved_base = current_dir;
            }

            let segments: Vec<&str> = normalized
                .split("::")
                .flat_map(|s| s.split('.'))
                .filter(|s| !s.is_empty())
                .collect();

            if !segments.is_empty() {
                // Check if it's a cross-crate import in workspace
                let first_seg = segments[0];
                let workspace_crate_src = std::path::Path::new(root_dir.value())
                    .join("crates")
                    .join(first_seg.replace('_', "-"))
                    .join("src");
                if workspace_crate_src.exists() {
                    let sub_segments = &segments[1..];
                    for len in (1..=sub_segments.len()).rev() {
                        let module_path = sub_segments[..len].join("/");
                        if let Some(target) = try_resolve_candidates(
                            &workspace_crate_src.to_string_lossy(),
                            &module_path,
                            file_set,
                        ) {
                            return Some(target);
                        }
                    }
                }
            }

            // Fallback: resolve relative to the resolved_base directory
            for len in (1..=segments.len()).rev() {
                let module_path = segments[..len].join("/");
                if let Some(target) =
                    try_resolve_candidates(&resolved_base.to_string_lossy(), &module_path, file_set)
                {
                    return Some(target);
                }
            }
        }
    }

    // 3. Fallback standard module resolution (Python or other language standard import)
    let segments: Vec<&str> = module.split('.').filter(|s| !s.is_empty()).collect();
    for len in (1..=segments.len()).rev() {
        let module_path = segments[..len].join("/");
        if let Some(target) = try_resolve_candidates(root_dir.value(), &module_path, file_set) {
            return Some(target);
        }
    }

    None
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
        let file_set: HashSet<String> =
            files.values.iter().map(|f| f.value().to_string()).collect();

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files.values {
            let imports = match analyzer.parser().extract_imports(file) {
                Ok(imp) => imp,
                Err(_) => continue,
            };

            for imp in imports.values {
                if let Some(target) = resolve_import_to_file(&imp.module, file, root_dir, &file_set)
                {
                    adjacency
                        .entry(file.value().to_string())
                        .or_default()
                        .push(target);
                }
            }
        }

        let mut global_visited: HashSet<String> = HashSet::new();
        let mut reported_cycles: HashSet<String> = HashSet::new();

        for file in &files.values {
            let file_str = file.value().to_string();
            if global_visited.contains(&file_str) {
                continue;
            }

            let mut in_stack: HashSet<String> = HashSet::new();
            let mut path: Vec<String> = Vec::new();

            if let Some(cycle) = find_cycle_dfs(
                &file_str,
                &adjacency,
                &mut global_visited,
                &mut in_stack,
                &mut path,
            ) {
                let mut unique_nodes = cycle[..cycle.len() - 1].to_vec();

                if !unique_nodes.is_empty() {
                    let min_idx = unique_nodes
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, val)| val)
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);

                    unique_nodes.rotate_left(min_idx);
                    unique_nodes.push(unique_nodes[0].clone());

                    let cycle_display = unique_nodes.join(" -> ");

                    if reported_cycles.insert(cycle_display.clone()) {
                        if let Ok(cycle_file) = FilePath::new(unique_nodes[0].clone()) {
                            results.push(LintResult {
                                file: cycle_file,
                                line: crate::common::taxonomy_common_vo::LineNumber::new(1),
                                column: crate::common::taxonomy_common_vo::ColumnNumber::new(0),
                                code: ErrorCode::raw("AES205"),
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
                                related_locations:
                                    crate::common::taxonomy_lint_vo::LocationList::new(),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli_commands::taxonomy_result_vo::LintResultList;
    use crate::code_analysis::taxonomy_import_source_vo::{
        ImportInfo, ImportInfoList, PrimitiveViolationList,
    };
    use crate::common::taxonomy_common_vo::{BooleanVO, Count, LineNumber, PatternList};
    use crate::common::taxonomy_definition_vo::LayerMapVO;
    use crate::common::taxonomy_layer_vo::LayerNameVO;
    use crate::common::taxonomy_name_vo::SymbolName;
    use crate::common::taxonomy_suggestion_vo::MetadataVO;
    use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
    use crate::file_system::contract_system_port::IFileSystemPort;
    use crate::import_rules::contract_rule_protocol::IAnalyzer;
    use crate::mcp_server::taxonomy_job_vo::{ResponseData, SuccessStatus};
    use crate::source_parsing::contract_parser_port::ISourceParserPort;
    use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
    use crate::source_parsing::taxonomy_parser_error::SourceParserError;
    use crate::source_parsing::taxonomy_path_vo::FilePath;
    use crate::source_parsing::taxonomy_paths_vo::FilePathList;
    use std::collections::HashMap;
    use std::fs;

    struct MockSourceParserPort {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockSourceParserPort {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(imp_list) = self.imports.get(path.value()) {
                for imp in imp_list {
                    list.push(ImportInfo::new(LineNumber::new(1), imp.clone()));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(&self, _path: &FilePath) -> Result<ResponseData, SourceParserError> {
            Ok(ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            })
        }
        fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
            ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            }
        }
        fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &PrimitiveTypeList,
        ) -> PrimitiveViolationList {
            PrimitiveViolationList::new()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            ImportInfoList::new()
        }
        fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
            Ok(MetadataVO::new(HashMap::new()))
        }
        fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_control_flow_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        fn is_barrel_file(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_stem(&self, _path: &FilePath) -> SymbolName {
            SymbolName::new(String::new())
        }
        fn is_entry_point(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_supported_extensions(&self) -> PatternList {
            PatternList { values: vec![] }
        }
    }

    struct MockFileSystemPort {
        _dummy: bool,
    }
    #[async_trait::async_trait]
    impl IFileSystemPort for MockFileSystemPort {
        async fn walk(
            &self,
            _path: &FilePath,
            _ignored_patterns: Option<&PatternList>,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn is_directory(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn is_file(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_relative_path(&self, _path: &FilePath, _start: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_text(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
        async fn get_line_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        async fn exists(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_parent(&self, _path: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn write_text(
            &self,
            _path: &FilePath,
            _content: &crate::common::taxonomy_source_vo::ContentString,
            _mode: Option<&crate::common::taxonomy_layer_vo::Identity>,
        ) -> Result<SuccessStatus, crate::file_system::taxonomy_filesystem_error::FileSystemError>
        {
            Ok(SuccessStatus::new(true))
        }
        async fn glob(
            &self,
            _pattern: &crate::common::taxonomy_layer_vo::Identity,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn get_cwd(&self) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn get_basename(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_layer_vo::Identity {
            crate::common::taxonomy_layer_vo::Identity::new("")
        }
        async fn path_join(
            &self,
            _parts: &[crate::common::taxonomy_layer_vo::Identity],
        ) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_file(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
    }

    struct MockAnalyzer {
        parser: MockSourceParserPort,
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
        fs: MockFileSystemPort,
    }

    impl IAnalyzer for MockAnalyzer {
        fn config(&self) -> &ArchitectureConfig {
            &self.config
        }
        fn layer_map(&self) -> &LayerMapVO {
            &self.layer_map
        }
        fn fs(&self) -> &dyn IFileSystemPort {
            &self.fs
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            None
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            None
        }
    }

    #[tokio::test]
    async fn test_check_cycles_detection() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(!results.values.is_empty());
        assert_eq!(&*results.values[0].code, "AES205");
    }

    #[tokio::test]
    async fn test_check_cycles_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            !results.values.is_empty(),
            "Should detect self circular dependency"
        );
        let result = &results.values[0];
        assert_eq!(&*result.code, "AES205");
        assert!(result.message.value().contains("/src/a.rs -> /src/a.rs"));
    }

    struct MockParserForCycle {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockParserForCycle {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(modules) = self.imports.get(path.value()) {
                for (i, module) in modules.iter().enumerate() {
                    list.push(ImportInfo::new(
                        LineNumber::new((i + 1) as i64),
                        module.clone(),
                    ));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(
            &self,
            _path: &FilePath,
        ) -> Result<crate::mcp_server::taxonomy_job_vo::ResponseData, SourceParserError> {
            todo!()
        }
        fn get_class_attributes(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::ResponseData {
            todo!()
        }
        fn has_all_export(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList,
        ) -> crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
            todo!()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            todo!()
        }
        fn get_class_definitions(
            &self,
            _path: &FilePath,
        ) -> Result<crate::common::taxonomy_suggestion_vo::MetadataVO, SourceParserError> {
            todo!()
        }
        fn get_function_definitions(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn is_symbol_exported(
            &self,
            _path: &FilePath,
            _symbol: &crate::common::taxonomy_name_vo::SymbolName,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn get_class_methods(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_class_bases_map(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_assignment_targets(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_control_flow_count(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_common_vo::Count {
            todo!()
        }
        fn is_barrel_file(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_stem(&self, _path: &FilePath) -> crate::common::taxonomy_name_vo::SymbolName {
            todo!()
        }
        fn is_entry_point(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_supported_extensions(&self) -> crate::common::taxonomy_common_vo::PatternList {
            todo!()
        }
    }

    struct MockAnalyzerForCycle {
        parser: MockParserForCycle,
    }

    impl IAnalyzer for MockAnalyzerForCycle {
        fn config(&self) -> &ArchitectureConfig {
            todo!()
        }
        fn layer_map(&self) -> &LayerMapVO {
            todo!()
        }
        fn fs(&self) -> &dyn IFileSystemPort {
            todo!()
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_happy_path_no_cycles() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec![]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            results.values.is_empty(),
            "Expected no cycles, found: {:?}",
            results.values
        );
    }

    #[tokio::test]
    async fn test_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/a.rs"),
            "Expected A->A cycle, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_simple_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/b.rs -> /src/a.rs")
                || msg.contains("/src/b.rs -> /src/a.rs -> /src/b.rs"),
            "Got message: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_complex_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
    }

    #[tokio::test]
    async fn test_js_ts_relative_and_barrel_imports() {
        let temp_dir = std::env::temp_dir().join("js_ts_cycle_test");
        let src_dir = temp_dir.join("src");
        let components_dir = src_dir.join("components");
        let utils_dir = src_dir.join("utils");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&utils_dir).unwrap();

        let button_file = components_dir.join("button.ts");
        let index_file = utils_dir.join("index.ts");
        let helper_file = utils_dir.join("helper.ts");

        fs::write(&button_file, "").unwrap();
        fs::write(&index_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let button_str = button_file.to_string_lossy().to_string();
        let index_str = index_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(button_str.clone(), vec!["../utils".to_string()]);
        imports.insert(index_str.clone(), vec!["./helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["../components/button".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(button_str.clone()).unwrap(),
                FilePath::new(index_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected exactly 1 cycle, found: {:?}",
            results.values
        );
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("button.ts") && msg.contains("index.ts") && msg.contains("helper.ts"),
            "Got message: {}",
            msg
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_python_relative_imports() {
        let temp_dir = std::env::temp_dir().join("python_cycle_test");
        let pkg_dir = temp_dir.join("pkg");
        let sub_dir = pkg_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();

        let init_file = pkg_dir.join("__init__.py");
        let main_file = pkg_dir.join("main.py");
        let sub_init_file = sub_dir.join("__init__.py");
        let sub_module_file = sub_dir.join("module.py");

        fs::write(&init_file, "").unwrap();
        fs::write(&main_file, "").unwrap();
        fs::write(&sub_init_file, "").unwrap();
        fs::write(&sub_module_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_module_str = sub_module_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec![".sub.module".to_string()]);
        imports.insert(sub_module_str.clone(), vec!["..main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(sub_module_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_rust_crate_absolute_and_super_imports() {
        let temp_dir = std::env::temp_dir().join("rust_cycle_test");
        let src_dir = temp_dir.join("src");
        let sub_dir = src_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(temp_dir.join("Cargo.toml"), "").unwrap();

        let main_file = src_dir.join("main.rs");
        let sub_mod_file = sub_dir.join("mod.rs");
        let helper_file = sub_dir.join("helper.rs");

        fs::write(&main_file, "").unwrap();
        fs::write(&sub_mod_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_mod_str = sub_mod_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec!["crate::sub::helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["crate::sub".to_string()]);
        imports.insert(sub_mod_str.clone(), vec!["crate::main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
                FilePath::new(sub_mod_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
