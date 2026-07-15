// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit_once('.') {
            Some((_, ext)) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }

    /// Extract the file stem (basename without the last extension).
    pub fn stem(&self) -> String {
        let base = self.basename();
        if let Some(pos) = base.rfind('.') {
            base[..pos].to_string()
        } else {
            base
        }
    }

    /// Extract the suffix (word after the last underscore in the stem).
    pub fn suffix(&self) -> String {
        let st = self.stem();
        match st.rfind('_') {
            Some(pos) => st[pos + 1..].to_string(),
            None => String::new(),
        }
    }

    /// Detect language from extension.
    pub fn language(&self) -> crate::common::taxonomy_language_vo::Language {
        let ext = self.extension();
        match ext.as_str() {
            "py" => crate::common::taxonomy_language_vo::Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => {
                crate::common::taxonomy_language_vo::Language::JavaScript
            }
            "ts" | "tsx" | "mts" | "cts" => {
                crate::common::taxonomy_language_vo::Language::TypeScript
            }
            "rs" => crate::common::taxonomy_language_vo::Language::Rust,
            _ => crate::common::taxonomy_language_vo::Language::Unknown,
        }
    }

    /// Check if the file is a lintable source code file.
    pub fn is_lintable(&self) -> bool {
        matches!(
            self.language(),
            crate::common::taxonomy_language_vo::Language::Python
                | crate::common::taxonomy_language_vo::Language::JavaScript
                | crate::common::taxonomy_language_vo::Language::TypeScript
                | crate::common::taxonomy_language_vo::Language::Rust
        )
    }

    /// Check if the path should be ignored according to patterns.
    pub fn is_ignored(&self, ignored: &[String]) -> bool {
        let rel_path = &self.value;
        if rel_path.is_empty() {
            return false;
        }
        let segments: Vec<&str> = rel_path
            .split(['/', '\\'])
            .filter(|s| !s.is_empty())
            .collect();
        for pat in ignored {
            if pat.is_empty() {
                continue;
            }
            // (1) Absolute-style prefix "/foo" or "/foo/bar"
            if let Some(stripped) = pat.strip_prefix('/') {
                if stripped.is_empty() {
                    continue;
                }
                let pat_segments: Vec<&str> = stripped
                    .split(['/', '\\'])
                    .filter(|s| !s.is_empty())
                    .collect();
                if pat_segments.is_empty() {
                    continue;
                }
                let n_pat = pat_segments.len();
                let n_seg = segments.len();
                if n_seg < n_pat {
                    continue;
                }
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
                continue;
            }
            // (2) Suffix glob "*.ext" or ".ext"
            if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
                let suffix = if let Some(s) = pat.strip_prefix('*') {
                    s.trim_start_matches('.')
                } else {
                    pat.trim_start_matches('.')
                };
                if suffix.is_empty() {
                    continue;
                }
                let basename = segments.last().copied().unwrap_or_default();
                if basename.ends_with(suffix) {
                    return true;
                }
                continue;
            }
            // (3) Bare segment/pattern
            let pat_segments: Vec<&str> =
                pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
            if pat_segments.len() == 1 {
                if segments.contains(&pat_segments[0]) {
                    return true;
                }
            } else if pat_segments.len() > 1 {
                let n_pat = pat_segments.len();
                let n_seg = segments.len();
                if n_seg >= n_pat {
                    for start in 0..=(n_seg - n_pat) {
                        if segments[start..start + n_pat] == pat_segments[..] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Walk up from the path looking for workspace root markers.
    pub fn find_workspace_root(&self) -> Option<DirectoryPath> {
        let mut dir = std::path::Path::new(&self.value).to_path_buf();
        dir.pop();
        let dp = DirectoryPath::new(dir.to_string_lossy().to_string()).ok()?;
        dp.find_workspace_root()
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }

    /// Walk up from the directory looking for workspace root markers.
    pub fn find_workspace_root(&self) -> Option<DirectoryPath> {
        let mut dir = std::path::Path::new(&self.value).to_path_buf();
        if !dir.is_absolute() {
            dir = std::env::current_dir().ok()?.join(&dir);
        }
        loop {
            if dir.join("Cargo.toml").exists()
                || dir.join("crates").is_dir()
                || dir.join("packages").is_dir()
                || dir.join("modules").is_dir()
            {
                return DirectoryPath::new(dir.to_string_lossy().to_string()).ok();
            }
            if !dir.pop() {
                return None;
            }
        }
    }

    /// Recursively collect all lintable source files from this directory.
    pub fn collect_source_files(
        &self,
        root_dir: &std::path::Path,
        ignored: &[String],
    ) -> Vec<FilePath> {
        let mut files = Vec::new();
        let path = std::path::Path::new(&self.value);
        if path.is_file() {
            let relative_path = match path.strip_prefix(root_dir) {
                Ok(p) => p,
                Err(_) => path,
            };
            let rel_str = relative_path.to_string_lossy();
            let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default();
            if !fp.is_ignored(ignored) {
                if fp.is_lintable() {
                    files.push(fp);
                }
            }
            return files;
        }

        if let Ok(entries) = std::fs::read_dir(&self.value) {
            for entry in entries.flatten() {
                let path = entry.path();
                let relative_path = match path.strip_prefix(root_dir) {
                    Ok(p) => p,
                    Err(_) => &path,
                };
                let rel_str = relative_path.to_string_lossy();
                let fp_for_ignore = FilePath::new(rel_str.to_string()).unwrap_or_default();
                if fp_for_ignore.is_ignored(ignored) {
                    continue;
                }
                if path.is_dir() {
                    let dir_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    if dir_name == "tests" {
                        continue;
                    }
                    let sub_dir =
                        DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                    files.extend(sub_dir.collect_source_files(root_dir, ignored));
                } else if let Some(path_str) = path.to_str() {
                    if let Ok(fp) = FilePath::new(path_str.to_string()) {
                        if fp.is_lintable() {
                            files.push(fp);
                        }
                    }
                }
            }
        }
        files
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
