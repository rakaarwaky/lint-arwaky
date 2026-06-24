// PURPOSE: GitCommandsSurface — CLI surface for git integration
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::process::ExitCode;
use std::sync::Arc;

pub struct GitCommandsSurface {}

impl Default for GitCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    language_detector: Arc<
        dyn shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort,
    >,
    base: String,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    let project_path = FilePath::new(".".to_string()).unwrap_or_default();

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path)
        .await;

    let files: Vec<&shared::source_parsing::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
        .filter(|fp| {
            shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort::is_lintable(language_detector.as_ref(), fp)
        })
        .collect();

    println!("Base: {} (changed files)", base);
    println!("Files changed: {}", files.len());
    println!();

    let mut total_violations = 0;
    for f in &files {
        let results = code_analysis_linter.run_code_analysis_path(&f.value);
        let fv = results.len();
        total_violations += fv;
        if fv > 0 {
            println!("  {}  -> {} violation(s)", f.value, fv);
            for r in results.iter().take(3) {
                println!(
                    "    {}:{} [{}] {}",
                    r.file.value(),
                    r.line.value(),
                    format!("{:?}", r.severity).to_uppercase(),
                    r.message.value()
                );
            }
        } else {
            println!("  {}  -> clean", f.value);
        }
    }

    println!();
    println!(
        "{} violations across {} changed files",
        total_violations,
        files.len()
    );
    ExitCode::SUCCESS
}
