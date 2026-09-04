// Unit tests — skills language relevance and filtering for init command.
use dispatcher_lint_arwaky::surface_setup_action::{collect_init, is_skill_relevant_for_languages};
use shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use shared::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::project_setup::contract_setup_protocol::PreFlightResult;
use shared::project_setup::{
    CreateConfigDirResult, EMBEDDED_SKILLS, EmbeddedSkillVO, ProjectLanguageVO, ProjectLanguagesVO,
    SetupError, SetupManagementAggregate, WriteConfigResult,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[test]
fn test_python_only_project_skill_filtering() {
    let langs = ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("python")]);

    // Python skills must be included
    assert!(is_skill_relevant_for_languages(Some("python"), &langs));

    // Rust and TypeScript skills must NOT be included
    assert!(!is_skill_relevant_for_languages(Some("rust"), &langs));
    assert!(!is_skill_relevant_for_languages(Some("typescript"), &langs));

    // Generic / language-agnostic skills (None) must be included
    assert!(is_skill_relevant_for_languages(None, &langs));
}

#[test]
fn test_rust_only_project_skill_filtering() {
    let langs = ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("rust")]);

    assert!(is_skill_relevant_for_languages(Some("rust"), &langs));
    assert!(!is_skill_relevant_for_languages(Some("python"), &langs));
    assert!(!is_skill_relevant_for_languages(Some("typescript"), &langs));
    assert!(is_skill_relevant_for_languages(None, &langs));
}

#[test]
fn test_typescript_only_project_skill_filtering() {
    let langs_js = ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("javascript")]);
    assert!(is_skill_relevant_for_languages(
        Some("typescript"),
        &langs_js
    ));
    assert!(!is_skill_relevant_for_languages(Some("python"), &langs_js));
    assert!(!is_skill_relevant_for_languages(Some("rust"), &langs_js));
    assert!(is_skill_relevant_for_languages(None, &langs_js));

    let langs_ts = ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("typescript")]);
    assert!(is_skill_relevant_for_languages(
        Some("typescript"),
        &langs_ts
    ));
    assert!(!is_skill_relevant_for_languages(Some("python"), &langs_ts));
    assert!(!is_skill_relevant_for_languages(Some("rust"), &langs_ts));
}

#[test]
fn test_multi_language_project_skill_filtering() {
    let langs = ProjectLanguagesVO::new(vec![
        ProjectLanguageVO::new("rust"),
        ProjectLanguageVO::new("python"),
    ]);

    assert!(is_skill_relevant_for_languages(Some("rust"), &langs));
    assert!(is_skill_relevant_for_languages(Some("python"), &langs));
    assert!(!is_skill_relevant_for_languages(Some("typescript"), &langs));
    assert!(is_skill_relevant_for_languages(None, &langs));
}

#[test]
fn test_empty_detected_languages_installs_all_by_default() {
    let empty_langs = ProjectLanguagesVO::new(vec![]);

    assert!(is_skill_relevant_for_languages(
        Some("python"),
        &empty_langs
    ));
    assert!(is_skill_relevant_for_languages(Some("rust"), &empty_langs));
    assert!(is_skill_relevant_for_languages(
        Some("typescript"),
        &empty_langs
    ));
    assert!(is_skill_relevant_for_languages(None, &empty_langs));
}

#[test]
fn test_embedded_skills_constants_catalog() {
    assert_eq!(EMBEDDED_SKILLS.len(), 39);

    let mut py_count = 0;
    let mut rs_count = 0;
    let mut ts_count = 0;
    let mut generic_count = 0;

    for skill in EMBEDDED_SKILLS {
        assert!(!skill.name().is_empty());
        assert!(!skill.relative_path().is_empty());
        assert!(!skill.content().is_empty());

        match skill.language() {
            Some("python") => {
                assert!(skill.name().ends_with("-python"));
                py_count += 1;
            }
            Some("rust") => {
                assert!(skill.name().ends_with("-rust"));
                rs_count += 1;
            }
            Some("typescript") => {
                assert!(skill.name().ends_with("-typescript"));
                ts_count += 1;
            }
            None => {
                generic_count += 1;
            }
            Some(other) => panic!("Unexpected language: {}", other),
        }
    }

    assert_eq!(py_count, 12);
    assert_eq!(rs_count, 12);
    assert_eq!(ts_count, 12);
    assert_eq!(generic_count, 3);
}

// ── Mock for collect_init integration test ─────────────────
struct MockSetupOrchestrator {
    detected: ProjectLanguagesVO,
}

impl SetupManagementAggregate for MockSetupOrchestrator {
    fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    fn generate_env(&self, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO::new("")
    }
    fn generate_mcp_config(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_claude(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_cursor(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_windsurf(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_copilot(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_hermes(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_vscode(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn mcp_config_all(&self) -> McpConfigVO {
        McpConfigVO::new(serde_json::json!({}))
    }
    fn install_python_adapters(&self) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    fn install_javascript_adapters(&self, _sudo: bool) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    fn detect_language(&self) -> Option<ProjectLanguageVO> {
        self.detected.values.first().cloned()
    }
    fn detect_languages(&self) -> ProjectLanguagesVO {
        self.detected.clone()
    }
    fn get_config_template(&self, _language: &str) -> Result<&'static str, SetupError> {
        Ok("rules: []")
    }
    fn pre_flight_check(&self) -> PreFlightResult {
        vec![]
    }
    fn get_embedded_skills(&self) -> &'static [EmbeddedSkillVO] {
        EMBEDDED_SKILLS
    }
    fn write_config_file(&self, filename: &str, _content: &str) -> WriteConfigResult {
        Ok(DescriptionVO::new(format!("wrote {filename}")))
    }
    fn create_global_config_dir(&self) -> CreateConfigDirResult {
        Ok(PathBuf::from("/tmp/mock-config"))
    }
    fn file_exists(&self, _path: &str) -> bool {
        false
    }
}

#[derive(Default)]
struct RecordingFilesystem {
    written_files: Mutex<HashMap<String, String>>,
}

impl IFileSystemIOProtocol for RecordingFilesystem {
    fn path_exists(&self, _path: &Path) -> bool {
        false
    }
    fn is_dir(&self, _path: &Path) -> bool {
        false
    }
    fn is_file(&self, _path: &Path) -> bool {
        false
    }
    fn should_ignore(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
        _ignored: &[String],
    ) -> bool {
        false
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(
        &self,
        path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_path_vo::FilePath {
        path.clone()
    }
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
    fn metadata(&self, _path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn symlink_metadata(&self, _path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_source_file(&self, _path: &Path) -> bool {
        false
    }
    fn is_source_ext(
        &self,
        _ext: &shared::filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        false
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_python_file(&self, _path: &Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _dir: &Path,
        _ignored: &shared::common::taxonomy_common_vo::PatternList,
    ) -> Vec<PathBuf> {
        vec![]
    }
    fn is_ignored_dir(
        &self,
        _dir: &Path,
        _ignored: &shared::common::taxonomy_common_vo::PatternList,
    ) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(
        &self,
        _path: &Path,
    ) -> Result<shared::common::taxonomy_source_vo::ContentString, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "mock"))
    }
    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        let mut map = self.written_files.lock().unwrap();
        map.insert(path.to_string_lossy().to_string(), content.to_string());
        Ok(())
    }
    fn copy_file(
        &self,
        _src: &Path,
        _dst: &Path,
    ) -> Result<shared::filesystem::taxonomy_filesystem_vo::ByteCount, std::io::Error> {
        Ok(shared::filesystem::taxonomy_filesystem_vo::ByteCount::new(
            0,
        ))
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[test]
fn test_collect_init_python_only_skips_rust_and_ts_skills() {
    let mock_orch = Arc::new(MockSetupOrchestrator {
        detected: ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("python")]),
    });
    let mock_fs = Arc::new(RecordingFilesystem::default());

    let items = collect_init(mock_orch, mock_fs.clone());
    assert!(!items.is_empty());

    let written = mock_fs.written_files.lock().unwrap();

    // Verify Python skills ARE installed
    assert!(
        written.keys().any(|k| k.contains("add-docs-python")),
        "add-docs-python should be installed"
    );
    assert!(
        written.keys().any(|k| k.contains("lint-arwaky-python")),
        "lint-arwaky-python should be installed"
    );

    // Verify common skills ARE installed
    assert!(
        written.keys().any(|k| k.contains("create-skill-all")),
        "create-skill-all should be installed"
    );
    assert!(
        written.keys().any(|k| k.contains("setup-ci-quality-gates")),
        "setup-ci-quality-gates should be installed"
    );

    // Verify Rust and TypeScript skills are NOT installed
    assert!(
        !written.keys().any(|k| k.contains("add-docs-rust")),
        "add-docs-rust must NOT be installed in a python-only project"
    );
    assert!(
        !written.keys().any(|k| k.contains("lint-arwaky-rust")),
        "lint-arwaky-rust must NOT be installed in a python-only project"
    );
    assert!(
        !written.keys().any(|k| k.contains("add-docs-typescript")),
        "add-docs-typescript must NOT be installed in a python-only project"
    );
    assert!(
        !written.keys().any(|k| k.contains("lint-arwaky-typescript")),
        "lint-arwaky-typescript must NOT be installed in a python-only project"
    );

    // Total skills written: 12 (python) + 3 (generic) = 15
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, 15);
}

#[test]
fn test_collect_init_rust_only_skips_python_and_ts_skills() {
    let mock_orch = Arc::new(MockSetupOrchestrator {
        detected: ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("rust")]),
    });
    let mock_fs = Arc::new(RecordingFilesystem::default());

    let items = collect_init(mock_orch, mock_fs.clone());
    assert!(!items.is_empty());

    let written = mock_fs.written_files.lock().unwrap();

    // Verify Rust skills ARE installed
    assert!(written.keys().any(|k| k.contains("add-docs-rust")));
    assert!(written.keys().any(|k| k.contains("lint-arwaky-rust")));

    // Verify Python and TypeScript skills are NOT installed
    assert!(!written.keys().any(|k| k.contains("add-docs-python")));
    assert!(!written.keys().any(|k| k.contains("add-docs-typescript")));

    // Total skills written: 12 (rust) + 3 (generic) = 15
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, 15);
}
