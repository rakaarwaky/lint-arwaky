// PURPOSE: PyRuffAdapter — ILinterAdapterProtocol implementation for Ruff linter integration
//
// Executes `ruff check --output-format=json` as a subprocess and parses
// the JSON output. Ruff outputs a JSON array of diagnostics with file paths,
// line numbers, severity levels, and rule codes.
//
// Key handling:
//   - Falls back to parent directory if target is a file (Ruff requires a directory)
//   - Searches for pyproject.toml to determine the correct working directory
//   - Maps Ruff severity levels (error/warning/info) to AES severity
//   - Converts relative Ruff paths to absolute project paths

use serde_json::Value;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::ErrorMessage;
use shared::common::taxonomy_adapter_error::AdapterError;
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

pub struct RuffAdapter {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.filesystem.is_python_file_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value().to_string(),
            "--exclude".to_string(),
            "tests".to_string(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .map_err(crate::convert_executor_error)?;

        let stdout = &response.stdout;
        // Empty output — tool found nothing to report (or no applicable files)
        if stdout.trim().is_empty() {
            return Ok(LintResultList::new(vec![]));
        }
        let findings: Vec<Value> = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(e) => {
                return Err(LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(format!(
                        "Failed to parse ruff JSON output: {}. Output was: {:?}",
                        e,
                        stdout.chars().take(200).collect::<String>()
                    )),
                )));
            }
        };
        let mut results = Vec::new();

        for f in &findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let row = f
                .get("location")
                .and_then(|l| l.get("row"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let col = f
                .get("location")
                .and_then(|l| l.get("column"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let code = f.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
            let message = f
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let severity_str = f
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let resolved = resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(row),
                column: ColumnNumber::new(col),
                code: ErrorCode::raw(code),
                message: LintMessage::new(message),
                source: Some(self.name()),
                severity: self.map_severity(severity_str, code),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value().to_string(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let _ = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .map_err(crate::convert_executor_error)?;
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RuffAdapter {
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
            None => "ruff".to_string(),
        }
    }

    fn map_severity(&self, _severity: &str, code: &str) -> Severity {
        // FR-004: Ruff severity mapping is code-based, not tool-severity-based.
        if code == "E999" || code.starts_with('S') {
            Severity::CRITICAL // syntax error (E999) or security rules (S1xx)
        } else if code == "F401" {
            Severity::MEDIUM // unused import
        } else if (code.starts_with('F')
            && code.len() >= 3
            && code[1..]
                .parse::<u32>()
                .is_ok_and(|n| (800..900).contains(&n)))
            || (code.starts_with('B')
                && code.len() >= 3
                && code[1..]
                    .parse::<u32>()
                    .is_ok_and(|n| (1..100).contains(&n)))
        {
            Severity::HIGH // F8xx: undefined name, B0xx: bugbear
        } else if (code.starts_with('E')
            && code.len() >= 3
            && code[1..]
                .parse::<u32>()
                .is_ok_and(|n| (100..200).contains(&n) || (500..600).contains(&n)))
            || (code.starts_with('W')
                && code.len() >= 3
                && code[1..]
                    .parse::<u32>()
                    .is_ok_and(|n| (200..300).contains(&n)))
        {
            Severity::LOW // E1xx: indentation, E5xx: line length, W2xx: whitespace
        } else {
            Severity::MEDIUM // default
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capabilities_py_ruff_adapter::RuffAdapter;
    use shared::common::taxonomy_severity_vo::Severity;
    use std::sync::Arc;

    struct MockFilesystem;
    impl shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate for MockFilesystem {
        fn file_list(&self) -> &[shared::filesystem::taxonomy_filesystem_vo::FileEntry] {
            &[]
        }
        fn read_cached(&self, _: &FilePath) -> shared::common::taxonomy_source_vo::ContentString {
            shared::common::taxonomy_source_vo::ContentString::raw("")
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

    fn make_adapter() -> RuffAdapter {
        use shared::external_lint::IExternalLintExecutorProtocol;

        struct EmptyLintExecutor;
        impl IExternalLintExecutorProtocol for EmptyLintExecutor {
            fn exec_cmd_scan(
                &self,
                _: Vec<String>,
                _: shared::common::taxonomy_path_vo::FilePath,
                _: f64,
                _: Option<shared::common::taxonomy_adapter_name_vo::AdapterName>,
                _: &shared::common::taxonomy_path_vo::FilePath,
            ) -> Result<
                shared::common::taxonomy_response_data_vo::ResponseData,
                shared::common::taxonomy_operation_error::LinterOperationError,
            > {
                Ok(shared::common::taxonomy_response_data_vo::ResponseData::default())
            }
            fn exec_cmd_adapter(
                &self,
                _: Vec<String>,
                _: shared::common::taxonomy_path_vo::FilePath,
                _: f64,
                _: shared::common::taxonomy_adapter_name_vo::AdapterName,
            ) -> Result<
                shared::common::taxonomy_response_data_vo::ResponseData,
                shared::common::taxonomy_operation_error::LinterOperationError,
            > {
                Ok(shared::common::taxonomy_response_data_vo::ResponseData::default())
            }
            fn js_apply_fix(
                &self,
                _: &shared::common::taxonomy_path_vo::FilePath,
                _: &str,
                _: &str,
            ) -> Result<
                shared::common::taxonomy_message_vo::ComplianceStatus,
                shared::common::taxonomy_operation_error::LinterOperationError,
            > {
                Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
                    false,
                ))
            }
        }

        let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
        RuffAdapter::new(executor, None, Arc::new(MockFilesystem))
    }

    // ─── FRD-004: Ruff severity mapping per code ───

    #[test]
    fn e999_syntax_error_maps_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("error", "E999"), Severity::CRITICAL);
    }

    #[test]
    fn security_rules_map_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "S105"), Severity::CRITICAL);
        assert_eq!(adapter.map_severity("warning", "S602"), Severity::CRITICAL);
        assert_eq!(adapter.map_severity("error", "S101"), Severity::CRITICAL);
    }

    #[test]
    fn f8xx_undefined_name_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "F821"), Severity::HIGH);
        assert_eq!(adapter.map_severity("warning", "F811"), Severity::HIGH);
    }

    #[test]
    fn b0xx_bugbear_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "B006"), Severity::HIGH);
        assert_eq!(adapter.map_severity("warning", "B007"), Severity::HIGH);
    }

    #[test]
    fn f401_unused_import_maps_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "F401"), Severity::MEDIUM);
    }

    #[test]
    fn e1xx_indentation_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "E111"), Severity::LOW);
        assert_eq!(adapter.map_severity("warning", "E117"), Severity::LOW);
    }

    #[test]
    fn e5xx_line_length_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "E501"), Severity::LOW);
    }

    #[test]
    fn w2xx_whitespace_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "W291"), Severity::LOW);
        assert_eq!(adapter.map_severity("warning", "W292"), Severity::LOW);
    }

    #[test]
    fn unknown_code_defaults_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "C999"), Severity::MEDIUM);
        assert_eq!(adapter.map_severity("error", "XXXX"), Severity::MEDIUM);
    }
}
