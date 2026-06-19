// PURPOSE: CodeDuplicationAnalyzer — AES305: detect duplicate code blocks across Rust files
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::config_system::taxonomy_config_vo::default_aes_config;
use std::path::Path;
use std::process::ExitCode;

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

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(&self, path: Option<String>) -> ExitCode {
        let root = crate::agent_project_target_orchestrator::resolve_target(path);
        println!("Code Duplication Detection — {}", root);
        println!();

        let src = crate::agent_code_analysis_orchestrator::detect_source_dir(Path::new(&root));
        let config = default_aes_config();
        let ignored: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let min_lines: usize = 10;
        let mut blocks: std::collections::HashMap<String, Vec<(std::path::PathBuf, usize)>> =
            std::collections::HashMap::new();

        source_parsing::infrastructure_file_collector::walk_rs_files(
            &src,
            &mut |p| {
                if let Ok(c) = std::fs::read_to_string(&p) {
                    let lines: Vec<&str> = c.lines().collect();
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
            },
            &ignored,
        );

        let duplicates: Vec<_> = blocks.into_iter().filter(|(_, v)| v.len() > 1).collect();

        let total_duplicated_lines = duplicates.len() * min_lines;
        let mut total_loc = 0usize;
        source_parsing::infrastructure_file_collector::walk_rs_files(
            &src,
            &mut |p| {
                if let Ok(c) = std::fs::read_to_string(&p) {
                    total_loc += c.lines().count();
                }
            },
            &ignored,
        );

        println!("Duplicate blocks: {}", duplicates.len());
        println!(
            "Duplicated lines: {} / {} ({:.1}%)",
            total_duplicated_lines,
            total_loc,
            if total_loc > 0 {
                total_duplicated_lines as f64 / total_loc as f64 * 100.0
            } else {
                0.0
            }
        );
        println!("Min block length: {} lines", min_lines);
        println!();

        for (i, (_, locations)) in duplicates.iter().enumerate().take(10) {
            let sim = 100usize.saturating_sub(locations.len().saturating_sub(1) * 5);
            println!(
                "#{}  {}% similarity across {} files",
                i + 1,
                sim.min(100),
                locations.len()
            );
            for (p, line) in locations.iter().take(3) {
                println!("      {}:{}", p.display(), line + 1);
            }
            if locations.len() > 3 {
                println!("      ... and {} more locations", locations.len() - 3);
            }
        }
        ExitCode::SUCCESS
    }
}
