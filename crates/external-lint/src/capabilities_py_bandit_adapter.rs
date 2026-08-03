// PURPOSE: PyBanditAdapter — ILinterAdapterProtocol implementation for Bandit security scanner integration
//
// Runs `bandit -r <path> --format json --exit-zero` to scan Python files for
// security vulnerabilities. Parses JSON output to extract findings (filename,
// line_range, test_id, issue_text, severity).
//
// Key details:
//   - `--exit-zero` ensures bandit always exits 0 regardless of findings
//   - JSON output avoids fragile regex parsing
//   - Severity is directly mapped: HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW
//   - apply_fix always returns false (Bandit is a scanner, not a fixer)

use serde_json::Value;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_path_normalization::resolve_capabilities_path;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::quality_rules::LinterOperationError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BanditAdapter {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.filesystem.is_python_file_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "-r".to_string(),
            path.value().to_string(),
            "--exclude".to_string(),
            "tests".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .map_err(crate::convert_executor_error)?;

        let stdout = &response.stdout;
        let parsed: Value = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(_) => Value::Object(serde_json::Map::new()),
        };
        let findings = match parsed.get("results").and_then(|v| v.as_array()) {
            Some(arr) => arr.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();

        for f in &findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let line_number = f
                .get("line_number")
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let line_range = f
                .get("line_range")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
            let issue_text = f
                .get("issue_text")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let issue_severity = f
                .get("issue_severity")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");
            let issue_confidence = f
                .get("issue_confidence")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");

            let resolved = resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(line_number),
                column: ColumnNumber::new(line_range),
                code: ErrorCode::raw(test_id),
                message: LintMessage::new(issue_text),
                source: Some(self.name()),
                severity: self.map_severity(issue_severity, issue_confidence),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl BanditAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
            filesystem,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value().to_string(),
            None => "bandit".to_string(),
        }
    }

    fn map_severity(&self, severity: &str, confidence: &str) -> Severity {
        // FR-004: Bandit severity — HIGH confidence + HIGH severity → CRITICAL.
        match (severity, confidence) {
            ("HIGH", "HIGH") => Severity::CRITICAL,
            ("HIGH", _) => Severity::HIGH,
            ("MEDIUM", _) => Severity::MEDIUM,
            ("LOW", _) => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capabilities_py_bandit_adapter::BanditAdapter;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_severity_vo::Severity;
    use std::sync::Arc;

    struct MockFilesystem;
    impl shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate for MockFilesystem {
        fn file_list(&self) -> &[shared::filesystem::taxonomy_filesystem_vo::FileEntry] {
            &[]
        }
        fn read_cached(&self, _: &FilePath) -> shared::common::taxonomy_source_vo::ContentString {
            shared::common::taxonomy_source_vo::ContentString::new("")
        }
        fn get_file_content(&self, _: &std::path::Path) -> Option<String> {
            None
        }
        fn has_file(&self, _: &std::path::Path) -> bool {
            false
        }
        fn collect_file_entries(&self, _: &[String]) -> Vec<(std::path::PathBuf, String)> {
            vec![]
        }
        fn discover_source_files(&self, _: &std::path::Path, _: &[String]) -> Vec<String> {
            vec![]
        }
        fn read_file(&self, _: &std::path::Path) -> Option<String> {
            None
        }
        fn scan_directory(&self, _: &std::path::Path) -> Vec<String> {
            vec![]
        }
        fn discover_files(&self, _: &std::path::Path) -> Vec<String> {
            vec![]
        }
        fn collect_source_files(&self, _: &std::path::Path, _: &[String]) -> Vec<FilePath> {
            vec![]
        }
        fn read_lintable_file(&self, _: &str) -> Option<String> {
            None
        }
    }
    use shared::common::taxonomy_config_language_vo::ConfigLanguage;
    use shared::common::taxonomy_language_vo::Language;
    use shared::filesystem::taxonomy_filesystem_vo::{
        DefinitionEntry, FileEntry as FE, FileExtension, ImplEntry, ImportEntry, ParseWarning,
        ScanTiming, ToolName,
    };
    impl shared::filesystem::contract_parser_protocol::IParserProtocol for MockFilesystem {
        fn parse_warnings(&self) -> &[ParseWarning] {
            &[]
        }
        fn import_list(&self) -> &[ImportEntry] {
            &[]
        }
        fn parse_all(&self, _: &mut [FE]) {}
        fn imports_for(&self, _: &std::path::Path) -> Vec<ImportEntry> {
            vec![]
        }
        fn extract(&self, _: &std::path::Path, _: &str, _: Language) -> Vec<ImportEntry> {
            vec![]
        }
    }
    impl shared::filesystem::contract_graph_protocol::IGraphProtocol for MockFilesystem {
        fn build_graph(&self, _: &[ImportEntry], _: &[FE], _: &[DefinitionEntry], _: &[ImplEntry]) {
        }
        fn symbol_definitions(
            &self,
        ) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
            todo!()
        }
        fn implementations(&self) -> &std::collections::HashMap<String, Vec<std::path::PathBuf>> {
            todo!()
        }
        fn dependents(&self, _: &std::path::Path) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn dependencies(&self, _: &std::path::Path) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn reachable(&self, _: &std::path::Path, _: &std::path::Path) -> bool {
            false
        }
        fn reverse_links(
            &self,
        ) -> &std::collections::HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> {
            todo!()
        }
    }
    impl shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol for MockFilesystem {
        fn workspace_root(&self, _: &FilePath) -> Option<std::path::PathBuf> {
            None
        }
        fn find_workspace_root_from_path(
            &self,
            _: &std::path::Path,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn is_member_path(&self, _: &FilePath) -> bool {
            false
        }
        fn is_leaf_member_path(&self, _: &FilePath) -> bool {
            false
        }
        fn detect_source_dir(&self, _: &std::path::Path) -> std::path::PathBuf {
            todo!()
        }
        fn detect_language_from_path(&self, _: &str) -> ConfigLanguage {
            todo!()
        }
        fn check_wired_in_container(&self, _: &std::path::Path, _: &[String]) -> bool {
            false
        }
        fn resolve_orphan_module_path(
            &self,
            _: &std::path::Path,
            _: &std::path::Path,
            _: &str,
        ) -> Option<std::path::PathBuf> {
            None
        }
    }
    impl shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol
        for MockFilesystem
    {
        fn is_executable_in_path(&self, _: &ToolName) -> bool {
            false
        }
        fn is_binary_available(&self, _: &ToolName) -> bool {
            false
        }
        fn has_local_bin(&self, _: &std::path::Path, _: &ToolName) -> bool {
            false
        }
        fn resolve_js_cmd(
            &self,
            _: &ToolName,
            _: Vec<String>,
            _: &FilePath,
        ) -> Option<Vec<String>> {
            None
        }
        fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
        fn has_config_file(&self, _: &std::path::Path) -> bool {
            false
        }
        fn has_cargo_toml(&self, _: &FilePath) -> Option<FilePath> {
            None
        }
        fn has_cargo_lock(&self, _: &FilePath) -> Option<FilePath> {
            None
        }
        fn is_python_file_recursive(&self, _: &FilePath) -> bool {
            false
        }
        fn default_working_dir(&self, path: &FilePath) -> FilePath {
            path.clone()
        }
    }
    impl shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol for MockFilesystem {
        fn path_exists(&self, _: &std::path::Path) -> bool {
            false
        }
        fn is_dir(&self, _: &std::path::Path) -> bool {
            false
        }
        fn is_file(&self, _: &std::path::Path) -> bool {
            false
        }
        fn should_ignore(&self, _: &FilePath, _: &[String]) -> bool {
            false
        }
        fn canonicalize(&self, _: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn canonicalize_path_str(&self, _: &FilePath) -> String {
            String::new()
        }
        fn is_symlink(&self, _: &std::path::Path) -> bool {
            false
        }
        fn metadata(&self, _: &std::path::Path) -> Result<std::fs::Metadata, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn symlink_metadata(
            &self,
            _: &std::path::Path,
        ) -> Result<std::fs::Metadata, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
        }
        fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
            path
        }
        fn is_source_file(&self, _: &std::path::Path) -> bool {
            false
        }
        fn is_source_ext(&self, _: &FileExtension) -> bool {
            false
        }
        fn get_basename<'a>(&self, path: &'a str) -> &'a str {
            path
        }
        fn get_parent<'a>(&self, path: &'a str) -> &'a str {
            path
        }
        fn is_python_file(&self, _: &std::path::Path) -> bool {
            false
        }
        fn scan_directory_with_ignored(
            &self,
            _: &std::path::Path,
            _: &[String],
        ) -> Vec<std::path::PathBuf> {
            vec![]
        }
        fn is_ignored_dir(&self, _: &std::path::Path, _: &[String]) -> bool {
            false
        }
        fn read_dir_entries_as_pathbuf(
            &self,
            _: &std::path::Path,
        ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
            Ok(vec![])
        }
        fn read_to_string(&self, _: &std::path::Path) -> Result<String, std::io::Error> {
            Ok(String::new())
        }
        fn write_string(&self, _: &std::path::Path, _: &str) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn copy_file(
            &self,
            _: &std::path::Path,
            _: &std::path::Path,
        ) -> Result<u64, std::io::Error> {
            Ok(0)
        }
        fn create_dir_all(&self, _: &std::path::Path) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn remove_dir_all(&self, _: &std::path::Path) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn set_permissions(&self, _: &std::path::Path, _: u32) -> std::io::Result<()> {
            Ok(())
        }
        fn remove_file(&self, _: &std::path::Path) -> std::io::Result<()> {
            Ok(())
        }
        fn run_git_command(&self, _: &[&str], _: &str) -> (String, String, bool) {
            (String::new(), String::new(), false)
        }
        fn parse_output_lines(&self, _: &str) -> Vec<String> {
            vec![]
        }
        fn run_external_command_in(&self, _: &str, _: &[&str], _: &str) -> (String, String, bool) {
            (String::new(), String::new(), false)
        }
        fn timing(&self) -> &ScanTiming {
            todo!()
        }
    }

    fn make_adapter() -> BanditAdapter {
        use shared::common::taxonomy_adapter_name_vo::AdapterName;
        use shared::common::taxonomy_message_vo::ComplianceStatus;
        use shared::common::taxonomy_operation_error::LinterOperationError;
        use shared::common::taxonomy_path_vo::FilePath;
        use shared::common::taxonomy_response_data_vo::ResponseData;
        use shared::external_lint::IExternalLintExecutorProtocol;

        struct EmptyLintExecutor;
        impl IExternalLintExecutorProtocol for EmptyLintExecutor {
            fn exec_cmd_scan(
                &self,
                _: Vec<String>,
                _: FilePath,
                _: f64,
                _: Option<AdapterName>,
                _: &FilePath,
            ) -> Result<ResponseData, LinterOperationError> {
                Ok(ResponseData::default())
            }
            fn exec_cmd_adapter(
                &self,
                _: Vec<String>,
                _: FilePath,
                _: f64,
                _: AdapterName,
            ) -> Result<ResponseData, LinterOperationError> {
                Ok(ResponseData::default())
            }
            fn js_apply_fix(
                &self,
                _: &FilePath,
                _: &str,
                _: &str,
            ) -> Result<ComplianceStatus, LinterOperationError> {
                Ok(ComplianceStatus::new(false))
            }
        }

        let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
        BanditAdapter::new(executor, None, Arc::new(MockFilesystem))
    }

    #[test]
    fn high_confidence_high_severity_maps_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("HIGH", "HIGH"), Severity::CRITICAL);
    }

    #[test]
    fn high_severity_low_confidence_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("HIGH", "LOW"), Severity::HIGH);
        assert_eq!(adapter.map_severity("HIGH", "MEDIUM"), Severity::HIGH);
    }

    #[test]
    fn medium_severity_any_confidence_maps_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("MEDIUM", "HIGH"), Severity::MEDIUM);
        assert_eq!(adapter.map_severity("MEDIUM", "LOW"), Severity::MEDIUM);
    }

    #[test]
    fn low_severity_any_confidence_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("LOW", "HIGH"), Severity::LOW);
        assert_eq!(adapter.map_severity("LOW", "LOW"), Severity::LOW);
    }

    #[test]
    fn unknown_severity_defaults_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("UNKNOWN", "HIGH"), Severity::MEDIUM);
    }
}
