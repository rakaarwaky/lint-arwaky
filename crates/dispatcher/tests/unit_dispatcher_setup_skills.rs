// Unit tests — skills language relevance and filtering for init command.
use dispatcher_lint_arwaky::surface_setup_action::{collect_init, is_skill_relevant_for_languages};
use shared_common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared_common::taxonomy_lint_vo::ContentString;
use shared_common::taxonomy_message_vo::DescriptionVO;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::taxonomy_filesystem_request::FilesystemRequest;
use shared_filesystem::taxonomy_filesystem_response::FilesystemResponse;
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
/// without an update, and it is the failure `taxonomy_project_setup_constant.rs`
/// (manual update) used to produce silently — the generator wrote to a filename
/// no module declared, so the committed constant never moved and no test noticed.
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
        "skill file(s) on disk are not embedded — update crates/shared/src/project_setup/taxonomy_project_setup_constant.rs manually: {missing:?}"
    );
    assert!(
        stale.is_empty(),
        "embedded skill(s) no longer exist on disk — update crates/shared/src/project_setup/taxonomy_project_setup_constant.rs manually: {stale:?}"
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

/// Like `RecordingFilesystem`, but also records `CopyFile` requests and
/// serves a fake `.agents` directory listing for the XDG config path so
/// `collect_init` exercises its `copy_dir_all` branch.
#[derive(Default)]
struct RecordingCopyFilesystem {
    written_files: Mutex<HashMap<String, String>>,
    copied_files: Mutex<Vec<(String, String)>>,
}

impl IFilesystemAggregate for RecordingCopyFilesystem {
    fn execute(&self, request: FilesystemRequest) -> FilesystemResponse {
        match request {
            FilesystemRequest::WriteFile { path, content } => {
                let mut map = self.written_files.lock().unwrap();
                map.insert(path.to_string_lossy().to_string(), content);
                FilesystemResponse::OpOk { ok: true }
            }
            FilesystemRequest::CopyFile { src, dst } => {
                let mut copies = self.copied_files.lock().unwrap();
                copies.push((
                    src.to_string_lossy().to_string(),
                    dst.to_string_lossy().to_string(),
                ));
                FilesystemResponse::OpOk { ok: true }
            }
            FilesystemRequest::ReadDirEntries { dir } => {
                // Only the XDG .agents dir has entries; nested dirs report none.
                let base = dir
                    .to_string_lossy()
                    .strip_suffix("/.agents")
                    .map(|b| b.to_string());
                if base.is_some() {
                    let paths = [
                        format!("{}/skills", dir.display()),
                        format!("{}/prompts", dir.display()),
                        format!("{}/research", dir.display()),
                    ]
                    .into_iter()
                    .collect();
                    FilesystemResponse::Paths { paths }
                } else {
                    FilesystemResponse::OpOk { ok: true }
                }
            }
            _ => FilesystemResponse::OpOk { ok: true },
        }
    }
}

#[test]
fn test_collect_init_copies_agents_dirs_but_skips_skills_and_prompts() {
    let mock_orch = Arc::new(MockSetupOrchestrator {
        detected: ProjectLanguagesVO::new(vec![ProjectLanguageVO::new("python")]),
    });
    let mock_fs = Arc::new(RecordingCopyFilesystem::default());

    let items = collect_init(mock_orch, mock_fs.clone());
    assert!(!items.is_empty());

    let copied = mock_fs.copied_files.lock().unwrap();
    // `copy_dir_all` reports zero copied files when every listed entry is a
    // directory, so the XDG-copy step must not record a failure.
    assert!(
        !items.iter().any(|i| i.message.contains("copy error")),
        "XDG .agents copy must succeed, got: {items:?}"
    );
    assert!(
        !copied.iter().any(|(src, _)| src.contains("/prompts/")),
        "no .agents/prompts/ file may be copied into the target project: {copied:?}"
    );
    assert!(
        !copied
            .iter()
            .any(|(_, dst)| dst.contains("/prompts/") || dst.contains("/skills/")),
        "prompts and skills dirs are skipped: {copied:?}"
    );
}

impl IFilesystemAggregate for RecordingFilesystem {
    fn execute(&self, request: FilesystemRequest) -> FilesystemResponse {
        match request {
            FilesystemRequest::WriteFile { path, content } => {
                let mut map = self.written_files.lock().unwrap();
                map.insert(path.to_string_lossy().to_string(), content);
                FilesystemResponse::OpOk { ok: true }
            }
            FilesystemRequest::CreateDirAll { .. } => FilesystemResponse::OpOk { ok: true },
            FilesystemRequest::ReadFileResult { .. } => FilesystemResponse::Content {
                value: ContentString::default(),
            },
            FilesystemRequest::PathExists { .. } => {
                FilesystemResponse::PathExists { exists: false }
            }
            FilesystemRequest::CopyFile { .. }
            | FilesystemRequest::RemoveDirAll { .. }
            | FilesystemRequest::RemoveFile { .. }
            | FilesystemRequest::SetPermissions { .. }
            | FilesystemRequest::Canonicalize { .. }
            | FilesystemRequest::ReadDirEntries { .. }
            | FilesystemRequest::FileList
            | FilesystemRequest::ReadCached { .. }
            | FilesystemRequest::GetFileContent { .. }
            | FilesystemRequest::HasFile { .. }
            | FilesystemRequest::CollectFileEntries { .. }
            | FilesystemRequest::DiscoverSourceFiles { .. }
            | FilesystemRequest::DiscoverFilesInDirectories { .. }
            | FilesystemRequest::ReadFile { .. }
            | FilesystemRequest::ScanDirectory { .. }
            | FilesystemRequest::DiscoverFiles { .. }
            | FilesystemRequest::CollectSourceFiles { .. }
            | FilesystemRequest::ReadLintableFile { .. }
            | FilesystemRequest::UsedIdentifiers { .. }
            | FilesystemRequest::FileListSnapshot
            | FilesystemRequest::ImplementedTraitsMap
            | FilesystemRequest::BuildFileIndex { .. }
            | FilesystemRequest::BuildFileIndexWithIgnored { .. }
            | FilesystemRequest::BuildOrphanGraphContext { .. }
            | FilesystemRequest::FindWorkspaceRoot { .. }
            | FilesystemRequest::ResolvedImportList
            | FilesystemRequest::ExtendImportCache { .. }
            | FilesystemRequest::ImportListSnapshot
            | FilesystemRequest::UsedIdentifiersAll
            | FilesystemRequest::DetectProjectLanguages { .. } => {
                FilesystemResponse::OpOk { ok: true }
            }
        }
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
