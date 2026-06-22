// PURPOSE: FileCollector — taxonomy utility for collecting lintable source files from a directory tree
use crate::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Collect lintable source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &std::path::Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path.strip_prefix(root_dir).unwrap_or(&path);
            let rel_str = relative_path.to_string_lossy();
            if ignored.iter().any(|i| rel_str.contains(i.as_str())) {
                continue;
            }
            if path.is_dir() {
                let sub_dir =
                    DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                files.extend(collect_source_files(root_dir, &sub_dir, ignored));
            } else if let Some(path_str) = path.to_str() {
                if let Ok(fp) = FilePath::new(path_str.to_string()) {
                    let detector = LanguageDetector::new();
                    if detector.is_lintable(&fp) {
                        files.push(fp);
                    }
                }
            }
        }
    }
    files
}
