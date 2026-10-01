// PURPOSE: SetupLanguageDetector — capability for project language detection
//
// Implements ILanguageDetectionProtocol. Detects the primary language and all
// languages present in a project by scanning for marker files and source
// extensions. No default language is returned when nothing is detected (FR-003).

use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_project_setup::contract_setup_protocol::ILanguageDetectionProtocol;
use shared_project_setup::taxonomy_project_setup_vo::{ProjectLanguageVO, ProjectLanguagesVO};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// Business logic for language detection.
pub struct SetupLanguageDetector {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILanguageDetectionProtocol for SetupLanguageDetector {
    /// Detect the primary language. Returns None when no languages are detected
    /// (no default — per FR-003 "No default language" rule).
    fn detect_language(&self) -> Option<ProjectLanguageVO> {
        let langs = self.detect_languages();
        langs.values.into_iter().next()
    }

    /// Detect ALL languages present in the project (FR-003).
    /// Returns empty list when no languages found — no default language.
    fn detect_languages(&self) -> ProjectLanguagesVO {
        let mut found_rust = false;
        let mut found_python = false;
        let mut found_javascript = false;

        // Phase 1: Marker-based detection (fast, no filesystem scan)
        if std::path::Path::new("crates").exists() || std::path::Path::new("Cargo.toml").exists() {
            found_rust = true;
        }
        if std::path::Path::new("modules").exists()
            || std::path::Path::new("pyproject.toml").exists()
            || std::path::Path::new("setup.py").exists()
            || std::path::Path::new("requirements.txt").exists()
        {
            found_python = true;
        }
        if std::path::Path::new("packages").exists()
            || std::path::Path::new("package.json").exists()
            || std::path::Path::new("tsconfig.json").exists()
        {
            found_javascript = true;
        }

        // Phase 2: File-extension scan (shallow, depth-limited)
        if !(found_rust && found_python && found_javascript) {
            self.scan_source_extensions(
                std::path::Path::new("."),
                0,
                4,
                &mut found_rust,
                &mut found_python,
                &mut found_javascript,
            );
        }

        let mut langs = Vec::new();
        if found_rust {
            langs.push(ProjectLanguageVO::new("rust"));
        }
        if found_python {
            langs.push(ProjectLanguageVO::new("python"));
        }
        if found_javascript {
            langs.push(ProjectLanguageVO::new("javascript"));
        }
        // FR-003: No default language — return empty list when nothing detected
        ProjectLanguagesVO::new(langs)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SetupLanguageDetector {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    /// Walk the directory tree (depth-limited) looking for source file extensions.
    /// Sets the corresponding `found_*` flag to `true` when a match is found.
    /// Skips hidden dirs, `target/`, `node_modules/`, and `vendor/` for speed.
    fn scan_source_extensions(
        &self,
        dir: &std::path::Path,
        depth: usize,
        max_depth: usize,
        found_rust: &mut bool,
        found_python: &mut bool,
        found_javascript: &mut bool,
    ) {
        if depth > max_depth {
            return;
        }
        let entries = match self.io.read_dir_entries_as_pathbuf(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for path in entries {
            if path.is_dir() {
                let name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                if name.starts_with('.') || shared_common::DEFAULT_IGNORED_PATHS.contains(&name) {
                    continue;
                }
                self.scan_source_extensions(
                    &path,
                    depth + 1,
                    max_depth,
                    found_rust,
                    found_python,
                    found_javascript,
                );
                if *found_rust && *found_python && *found_javascript {
                    return;
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => *found_rust = true,
                    "py" => *found_python = true,
                    "ts" | "tsx" | "mts" | "cts" | "js" | "jsx" | "mjs" | "cjs" => {
                        *found_javascript = true
                    }
                    _ => {}
                }
                if *found_rust && *found_python && *found_javascript {
                    return;
                }
            }
        }
    }
}
