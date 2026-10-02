// Unit tests — skills language relevance and filtering for init command.
use dispatcher_lint_arwaky::surface_setup_action::{collect_init, is_skill_relevant_for_languages};
use shared_common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_project_setup::{
    EMBEDDED_SKILLS, EMBEDDED_SKILLS_COUNT, ISetupAggregate, ProjectLanguageVO, ProjectLanguagesVO,
    SetupRequest, SetupResponse,
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

    // The counts are derived rather than hard-coded. Hard-coding them made
    // adding a single skill file a three-line test edit, and the numbers went
    // stale silently — `HOW-TO-MAKE-DATA.md` sat in `crates/shared/skills/`
    // un-embedded for a year precisely because nothing failed when the catalog
    // and the directory disagreed. The `catalog_matches_the_skills_directory`
    // test below is the real guard; this one only checks internal coherence.
    assert_eq!(
        py_count + rs_count + ts_count + generic_count,
        EMBEDDED_SKILLS.len(),
        "every entry must fall into exactly one language bucket"
    );
    assert_eq!(EMBEDDED_SKILLS.len(), EMBEDDED_SKILLS_COUNT);

    // Every skill folder ships exactly one language-agnostic SKILL.md, and no
    // two skills share one. The count is derived from the catalog rather than
    // pinned to a literal, so adding a skill does not mean editing this test.
    let skill_md_names: std::collections::BTreeSet<&str> = EMBEDDED_SKILLS
        .iter()
        .filter(|s| s.relative_path().ends_with("SKILL.md"))
        .map(|s| s.name())
        .collect();
    let skill_md_entries = EMBEDDED_SKILLS
        .iter()
        .filter(|s| s.relative_path().ends_with("SKILL.md"))
        .count();

    assert_eq!(
        skill_md_entries,
        skill_md_names.len(),
        "no two skills may share a SKILL.md name"
    );

    // A skill is a folder, and a folder holds exactly one SKILL.md. Any name
    // appearing in the catalog without a SKILL.md is either the pack README or
    // a reference file that escaped its skill folder.
    let catalogued_names: std::collections::BTreeSet<&str> =
        EMBEDDED_SKILLS.iter().map(|s| s.name()).collect();
    let names_without_skill_md: std::collections::BTreeSet<&str> = catalogued_names
        .into_iter()
        .filter(|n| !skill_md_names.contains(n))
        .collect();
    assert_eq!(
        names_without_skill_md,
        std::collections::BTreeSet::from(["README"]),
        "only the skills README may be catalogued without a SKILL.md"
    );

    // The three language columns are symmetric — a language with fewer
    // references than the others would mean `init` installs an incomplete
    // skill for that language.
    assert_eq!(
        py_count, rs_count,
        "python and rust reference sets must be the same size"
    );
    assert_eq!(
        rs_count, ts_count,
        "rust and typescript reference sets must be the same size"
    );
}

/// The catalog must match `crates/shared/skills/` on disk, entry for entry.
///
/// This is the guard that would have caught `HOW-TO-MAKE-DATA.md` being added
/// without a regeneration, and it is the failure `tools/regenerate_skills.py`
/// used to produce silently — the generator wrote to a filename no module
/// declared, so the committed constant never moved and no test noticed.
#[test]
fn catalog_matches_the_skills_directory() {
    let skills_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("shared")
        .join("skills");
    let mut on_disk: Vec<String> = walk_md(&skills_dir)
        .into_iter()
        .map(|p| {
            p.strip_prefix(&skills_dir)
                .expect("path is under the skills dir")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();
    on_disk.sort();

    let mut embedded: Vec<String> = EMBEDDED_SKILLS
        .iter()
        .map(|s| s.relative_path().to_string())
        .collect();
    embedded.sort();

    let missing: Vec<&String> = on_disk.iter().filter(|f| !embedded.contains(f)).collect();
    let stale: Vec<&String> = embedded.iter().filter(|f| !on_disk.contains(f)).collect();

    assert!(
        missing.is_empty(),
        "skill file(s) on disk are not embedded — run `python3 tools/regenerate_skills.py`: {missing:?}"
    );
    assert!(
        stale.is_empty(),
        "embedded skill(s) no longer exist on disk — run `python3 tools/regenerate_skills.py`: {stale:?}"
    );
    assert_eq!(
        on_disk.len(),
        EMBEDDED_SKILLS_COUNT,
        "EMBEDDED_SKILLS_COUNT must equal the number of skill files"
    );
}

/// Recursively collect every `.md` path under `dir`.
fn walk_md(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(walk_md(&path));
        } else if path.extension().is_some_and(|e| e == "md") {
            found.push(path);
        }
    }
    found
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

    // Derived from the catalog rather than hard-coded: everything
    // language-agnostic, plus the python half of the polyglot skills.
    let expected = EMBEDDED_SKILLS
        .iter()
        .filter(|s| matches!(s.language(), None | Some("python")))
        .count();
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, expected);
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

    // Derived from the catalog rather than hard-coded: everything
    // language-agnostic, plus the rust half of the polyglot skills.
    let expected = EMBEDDED_SKILLS
        .iter()
        .filter(|s| matches!(s.language(), None | Some("rust")))
        .count();
    let skill_files_count = written
        .keys()
        .filter(|k| k.contains(".agents/skills/"))
        .count();
    assert_eq!(skill_files_count, expected);
}
