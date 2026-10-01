// Unit tests — skills language relevance and filtering for init command.
use dispatcher_lint_arwaky::surface_setup_action::{collect_init, is_skill_relevant_for_languages};
use shared_common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_project_setup::{
    EMBEDDED_SKILLS, ISetupAggregate, ProjectLanguageVO, ProjectLanguagesVO, SetupRequest,
    SetupResponse,
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
    assert_eq!(EMBEDDED_SKILLS.len(), 56);

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
                assert!(skill.relative_path().contains("PYTHON"));
                py_count += 1;
            }
            Some("rust") => {
                assert!(skill.relative_path().contains("RUST"));
                rs_count += 1;
            }
            Some("typescript") => {
                assert!(skill.relative_path().contains("TYPESCRIPT"));
                ts_count += 1;
            }
            None => {
                generic_count += 1;
            }
            Some(other) => panic!("Unexpected language: {}", other),
        }
    }

    // Each skill ships exactly one language-agnostic SKILL.md.
    assert_eq!(
        EMBEDDED_SKILLS
            .iter()
            .filter(|s| s.relative_path().ends_with("SKILL.md"))
            .count(),
        11
    );

    assert_eq!(py_count, 11);
    assert_eq!(rs_count, 11);
    assert_eq!(ts_count, 11);
    assert_eq!(generic_count, 23);
}

// ── Mock for collect_init integration test ─────────────────
struct MockSetupOrchestrator {
    detected: ProjectLanguagesVO,
}

impl ISetupAggregate for MockSetupOrchestrator {
    fn execute(&self, request: SetupRequest) -> SetupResponse {
        match request {
            SetupRequest::CheckHttp { .. } => SetupResponse::CheckHttp {
                status: SuccessStatus::new(true),
            },
            SetupRequest::GenerateEnv { .. } => SetupResponse::Env {
                content: EnvContentVO::new(""),
            },
            SetupRequest::GenerateMcpConfig
            | SetupRequest::McpConfigClaude
            | SetupRequest::McpConfigCursor
            | SetupRequest::McpConfigWindsurf
            | SetupRequest::McpConfigCopilot
            | SetupRequest::McpConfigHermes
            | SetupRequest::McpConfigVscode
            | SetupRequest::McpConfigAll => SetupResponse::Mcp {
                config: McpConfigVO::new(HashMap::new()),
            },
            SetupRequest::InstallPythonAdapters
            | SetupRequest::InstallJavascriptAdapters { .. } => SetupResponse::Installed {
                status: SuccessStatus::new(true),
            },
            SetupRequest::DetectLanguage => SetupResponse::Language {
                detected: self.detected.values.first().cloned(),
            },
            SetupRequest::DetectLanguages => SetupResponse::Languages {
                detected: self.detected.clone(),
            },
            SetupRequest::GetConfigTemplate { .. } => SetupResponse::Template {
                content: Ok(String::from("rules: []")),
            },
            SetupRequest::PreFlightCheck => SetupResponse::PreFlight { result: vec![] },
            SetupRequest::GetEmbeddedSkills => SetupResponse::Skills {
                skills: EMBEDDED_SKILLS.to_vec(),
            },
            SetupRequest::WriteConfigFile { filename, .. } => SetupResponse::ConfigWritten {
                result: Ok(DescriptionVO::new(format!("wrote {filename}"))),
            },
            SetupRequest::CreateGlobalConfigDir => SetupResponse::ConfigDir {
                result: Ok(PathBuf::from("/tmp/mock-config")),
            },
            SetupRequest::FileExists { .. } => SetupResponse::Exists { exists: false },
        }
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
        _path: &shared_common::taxonomy_path_vo::FilePath,
        _ignored: &[String],
    ) -> bool {
        false
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(
        &self,
        path: &shared_common::taxonomy_path_vo::FilePath,
    ) -> shared_common::taxonomy_path_vo::FilePath {
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
        _ext: &shared_filesystem::taxonomy_filesystem_vo::FileExtension,
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
        _ignored: &shared_common::taxonomy_common_vo::PatternList,
    ) -> Vec<PathBuf> {
        vec![]
    }
    fn is_ignored_dir(
        &self,
        _dir: &Path,
        _ignored: &shared_common::taxonomy_common_vo::PatternList,
    ) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        Ok(vec![])
    }
    fn read_to_string(
        &self,
        _path: &Path,
    ) -> Result<shared_common::taxonomy_source_vo::ContentString, std::io::Error> {
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
    ) -> Result<shared_filesystem::taxonomy_filesystem_vo::ByteCount, std::io::Error> {
        Ok(shared_filesystem::taxonomy_filesystem_vo::ByteCount::new(0))
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_permissions(
        &self,
        _path: &Path,
        _mode: shared_filesystem::taxonomy_filesystem_vo::FileMode,
    ) -> std::io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &Path) -> std::io::Result<()> {
        Ok(())
    }
    fn run_git_command(
        &self,
        _args: &[&str],
        _dir: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::GitCommandResult {
        shared_filesystem::taxonomy_filesystem_vo::GitCommandResult::new(
            String::new(),
            String::new(),
            false,
        )
    }
    fn parse_output_lines(
        &self,
        output: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::ParsedLines {
        shared_filesystem::taxonomy_filesystem_vo::ParsedLines::new(
            output.lines().map(String::from).collect(),
        )
    }
    fn run_external_command_in(
        &self,
        _name: &ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), false)
    }
    fn timing(&self) -> &shared_filesystem::taxonomy_filesystem_vo::ScanTiming {
        static T: shared_filesystem::taxonomy_filesystem_vo::ScanTiming =
            shared_filesystem::taxonomy_filesystem_vo::ScanTiming {
                walk_ms: 0,
                cache_ms: 0,
                parse_ms: 0,
                extract_ms: 0,
                graph_ms: 0,
                total_ms: 0,
            };
        &T
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

    // Verify the Python half of every polyglot skill IS installed
    assert!(
        written.keys().any(|k| k.contains("aes-taxonomy/SKILL.md")),
        "aes-taxonomy is language-agnostic and must be installed"
    );
    assert!(
        written
            .keys()
            .any(|k| k.contains("aes-lint-arwaky/references/HOW-TO-USE-LINT-PYTHON.md")),
        "the python reference of aes-lint-arwaky should be installed"
    );
    assert!(
        written
            .keys()
            .any(|k| k.contains("aes-taxonomy/references/HOW-TO-MAKE-PYTHON-TAXONOMY.md")),
        "the python reference of aes-taxonomy should be installed"
    );

    // Verify common skills ARE installed
    assert!(
        written.keys().any(|k| k.contains("aes-docs/SKILL.md")),
        "aes-docs should be installed"
    );
    assert!(
        written.keys().any(|k| k.contains("aes-migration")),
        "aes-migration should be installed"
    );

    // Verify Rust and TypeScript references are NOT installed
    assert!(
        !written.keys().any(|k| k.contains("HOW-TO-MAKE-RUST-")),
        "rust references must NOT be installed in a python-only project"
    );
    assert!(
        !written
            .keys()
            .any(|k| k.contains("HOW-TO-MAKE-TYPESCRIPT-")),
        "typescript references must NOT be installed in a python-only project"
    );

    // Total skill files written for python-only project: 23 (language-agnostic) + 11 (python refs) = 34
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, 34);
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

    // Verify the Rust half of every polyglot skill IS installed
    assert!(
        written
            .keys()
            .any(|k| k.contains("aes-lint-arwaky/references/HOW-TO-USE-LINT-RUST.md"))
    );
    assert!(
        written
            .keys()
            .any(|k| k.contains("aes-capabilities/references/HOW-TO-MAKE-RUST-CAPABILITIES.md"))
    );

    // Verify Python and TypeScript references are NOT installed
    assert!(!written.keys().any(|k| k.contains("HOW-TO-MAKE-PYTHON-")));
    assert!(
        !written
            .keys()
            .any(|k| k.contains("HOW-TO-MAKE-TYPESCRIPT-"))
    );

    // Total skill files written for rust-only project: 23 (language-agnostic) + 11 (rust refs) = 34
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, 34);
}
