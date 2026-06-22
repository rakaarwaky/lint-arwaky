// PURPOSE: CodeDuplicationAnalyzer — AES305: detect duplicate code blocks across all lintable files
// ALGORITHM:
//   1. Resolve target directory (default: ".")
//   2. Walk all lintable files via IFileSystemPort (handles ignored patterns)
//   3. For each file, read content and tokenize into lines
//   4. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   5. Use normalized window as hash key in blocks map; store (file, start_line)
//   6. After scan, filter blocks map to entries with >1 location (duplicates)
//   7. Build AesCodeAnalysisViolation::CodeDuplication for each duplicate block
//   8. Append summary violation with total stats
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::config_system::taxonomy_config_vo::default_aes_config;
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct CodeDuplicationAnalyzer {}

impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CodeDuplicationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeDuplicationAnalyzer {
    pub fn check_duplicates(&self, files: &[String], min_dup_lines: usize) -> Vec<AesCodeAnalysisViolation> {
        let mut blocks: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();
        let detector = LanguageDetector::new();
        let mut total_loc: usize = 0;

        for file_str in files {
            let fp = match FilePath::new(file_str.clone()) {
                Ok(f) => f,
                Err(_) => continue,
            };
            if !detector.is_lintable(&fp) {
                continue;
            }
            let p = PathBuf::from(&fp.value);
            let content = match std::fs::read_to_string(&fp.value) {
                Ok(c) => c,
                Err(_) => continue,
            };
            total_loc += content.lines().count();
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for w in lines.windows(min_dup_lines) {
                let key: String = w
                    .iter()
                    .map(|s| {
                        s.trim()
                            .chars()
                            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("|");
                let start_line = w.as_ptr() as usize - lines.as_ptr() as usize;
                blocks.entry(key).or_default().push((p.clone(), start_line));
            }
        }

        let duplicates: Vec<_> = blocks.into_iter().filter(|(_, v)| v.len() > 1).collect();

        let mut violations = Vec::new();
        for (_, locations) in &duplicates {
            let msg = format!(
                "Duplicate block found across {} files (similarity ~{}%). Locations: {}",
                locations.len(),
                100usize.saturating_sub(locations.len().saturating_sub(1) * 5).min(100),
                locations
                    .iter()
                    .take(3)
                    .map(|(p, l)| format!("{}:{}", p.display(), l + 1))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            violations.push(AesCodeAnalysisViolation::CodeDuplication {
                reason: Some(LintMessage::new(msg)),
            });
        }

        if !duplicates.is_empty() {
            let total_dup_lines = duplicates.len() * min_dup_lines;
            let pct = if total_loc > 0 {
                total_dup_lines as f64 / total_loc as f64 * 100.0
            } else {
                0.0
            };
            violations.push(AesCodeAnalysisViolation::CodeDuplication {
                reason: Some(LintMessage::new(format!(
                    "Summary: {} duplicate blocks ({} lines) out of {} total LOC ({:.1}%)",
                    duplicates.len(), total_dup_lines, total_loc, pct,
                ))),
            });
        }

        violations
    }
}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation> {
        let root = crate::agent_code_analysis_orchestrator::resolve_target(path);
        let src = crate::agent_code_analysis_orchestrator::detect_source_dir(Path::new(&root));
        let config = default_aes_config();
        let ignored = PatternList {
            values: config
                .ignored_paths
                .values
                .iter()
                .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
                .collect(),
        };
        let min_lines = config
            .rules
            .first()
            .map(|r| r.code_analysis.min_lines.value)
            .filter(|&v| v > 0)
            .map(|v| v as usize)
            .unwrap_or(10);

        let mut blocks: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();
        let detector = LanguageDetector::new();
        let mut total_loc: usize = 0;

        let src_fp = match FilePath::new(src.to_string_lossy().to_string()) {
            Ok(fp) => fp,
            Err(_) => return Vec::new(),
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        let all_files = rt.block_on(fs.walk(&src_fp, Some(&ignored)));

        for file_fp in &all_files.values {
            if !detector.is_lintable(file_fp) {
                continue;
            }
            let p = PathBuf::from(&file_fp.value);
            let content = match rt.block_on(fs.read_text(file_fp)) {
                Ok(c) => c.value,
                Err(_) => continue,
            };
            total_loc += content.lines().count();
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_lines {
                continue;
            }
            for w in lines.windows(min_lines) {
                let key: String = w
                    .iter()
                    .map(|s| {
                        s.trim()
                            .chars()
                            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("|");
                let start_line = w.as_ptr() as usize - lines.as_ptr() as usize;
                blocks.entry(key).or_default().push((p.clone(), start_line));
            }
        }

        let duplicates: Vec<_> = blocks.into_iter().filter(|(_, v)| v.len() > 1).collect();

        let mut violations = Vec::new();
        for (_, locations) in &duplicates {
            let msg = format!(
                "Duplicate block found across {} files (similarity ~{}%). Locations: {}",
                locations.len(),
                100usize
                    .saturating_sub(locations.len().saturating_sub(1) * 5)
                    .min(100),
                locations
                    .iter()
                    .take(3)
                    .map(|(p, l)| format!("{}:{}", p.display(), l + 1))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            violations.push(AesCodeAnalysisViolation::CodeDuplication {
                reason: Some(LintMessage::new(msg)),
            });
        }

        if !duplicates.is_empty() {
            let total_dup_lines = duplicates.len() * min_lines;
            let pct = if total_loc > 0 {
                total_dup_lines as f64 / total_loc as f64 * 100.0
            } else {
                0.0
            };
            violations.push(AesCodeAnalysisViolation::CodeDuplication {
                reason: Some(LintMessage::new(format!(
                    "Summary: {} duplicate blocks ({} lines) out of {} total LOC ({:.1}%)",
                    duplicates.len(),
                    total_dup_lines,
                    total_loc,
                    pct,
                ))),
            });
        }

        violations
    }
}
