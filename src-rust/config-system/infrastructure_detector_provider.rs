use crate::config_system::contract_detector_port::ILanguageDetectorPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: LanguageSource */ crate::project_setup::taxonomy_language_vo::LanguageSource;
use async_trait::async_trait;

pub struct LanguageDetectorProvider {}

impl LanguageDetectorProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ILanguageDetectorPort for LanguageDetectorProvider {
    async fn detect_language(&self, project_root: &FilePath) -> LanguageSource {
        let root = std::path::Path::new(&project_root.value);

        // Step 1: Check source directory (confidence 100)
        if root.join("src-rust").exists() {
            return LanguageSource::new("rust", 100, "directory");
        }
        if root.join("src-python").exists() {
            return LanguageSource::new("python", 100, "directory");
        }
        if root.join("src-javascript").exists() {
            return LanguageSource::new("javascript", 100, "directory");
        }

        // Step 2: Check manifest file (confidence 90)
        if root.join("Cargo.toml").exists() {
            return LanguageSource::new("rust", 90, "manifest");
        }
        if root.join("pyproject.toml").exists() || root.join("setup.py").exists() {
            return LanguageSource::new("python", 90, "manifest");
        }
        if root.join("package.json").exists() {
            return LanguageSource::new("javascript", 90, "manifest");
        }

        // Step 3: Check src/ + file extension (confidence 70)
        let src_dir = root.join("src");
        if src_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&src_dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        match ext.to_str().unwrap_or("") {
                            "rs" => return LanguageSource::new("rust", 70, "extension"),
                            "py" => return LanguageSource::new("python", 70, "extension"),
                            "js" | "ts" | "jsx" | "tsx" => {
                                return LanguageSource::new("javascript", 70, "extension")
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Step 4: Fallback (confidence 50) — scan all files
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    *counts
                        .entry(ext.to_str().unwrap_or("").to_string())
                        .or_insert(0) += 1;
                }
            }
        }

        let lang = if counts.get("rs").unwrap_or(&0) > counts.get("py").unwrap_or(&0) {
            "rust"
        } else if counts.get("py").unwrap_or(&0) > counts.get("js").unwrap_or(&0) {
            "python"
        } else {
            "rust"
        };

        LanguageSource::new(lang, 50, "fallback")
    }
}
