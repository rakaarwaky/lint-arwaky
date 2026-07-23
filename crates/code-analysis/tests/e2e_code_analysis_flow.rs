// PURPOSE: E2E tests — full scan lifecycle from path input through all
// checkers to formatted report output. Uses real filesystem (temp dir).

use code_analysis_lint_arwaky::CodeAnalysisContainer;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;

fn setup_temp_project(files: Vec<(&str, &str)>) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    for (name, content) in files {
        let file_path = src.join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&file_path, content).unwrap();
    }
    dir
}

fn make_aggregate() -> std::sync::Arc<dyn ICodeAnalysisAggregate> {
    CodeAnalysisContainer::new().code_analysis_linter()
}

// ─── E2E: Clean project produces zero violations ─────────────────────

#[test]
fn e2e_clean_project_zero_violations() {
    let clean_struct = r#"pub struct Widget {
    name: String,
    size: usize,
}

impl Widget {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }

    pub fn display(&self) -> String {
        format!("{}: {}", self.name, self.size)
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_widget.rs", clean_struct)]);

    let orch = make_aggregate();
    let results = orch.run_code_analysis_path(
        &FilePath::new(dir.path().join("src").to_str().unwrap().to_string()).unwrap(),
    );

    // A single clean file should produce no AES301-305 violations
    let arch_violations: Vec<_> = results
        .iter()
        .filter(|r| r.code.code().starts_with("AES3"))
        .collect();
    assert!(
        arch_violations.is_empty(),
        "Expected no violations, got: {:?}",
        arch_violations
    );
}

// ─── E2E: Project with bypass produces AES304 ────────────────────────

#[test]
fn e2e_bypass_project_produces_aes304() {
    let bypass_code = r#"pub struct Service {
    active: bool,
}

impl Service {
    pub fn run(&self) {
        let val = Some(42).unwrap();
        println!("{}", val);
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_service.rs", bypass_code)]);

    let orch = make_aggregate();
    let results = orch.run_code_analysis_path(
        &FilePath::new(dir.path().join("src").to_str().unwrap().to_string()).unwrap(),
    );

    let aes304: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES304")
        .collect();
    assert!(!aes304.is_empty(), "Expected AES304 violation for unwrap()");
}

// ─── E2E: Full report generation ─────────────────────────────────────

#[test]
fn e2e_full_report_generation() {
    let code = r#"pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new() -> Self {
        Self { x: 0 }
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_foo.rs", code)]);

    let container = CodeAnalysisContainer::new();
    let aggregate = container.code_analysis_linter();

    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let results = aggregate.run_code_analysis(&root);
    let report = aggregate.format_report(&results, &root);

    assert!(report.value.contains("AES Architecture Compliance Report"));
    assert!(report.value.contains("Violations:"));
}

// ─── E2E: Score calculation after scan ───────────────────────────────

#[test]
fn e2e_score_after_scan() {
    let code = r#"pub struct Bar {
    y: f64,
}

impl Bar {
    pub fn compute(&self) -> f64 {
        self.y * 2.0
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_bar.rs", code)]);

    let orch = make_aggregate();
    let results = orch.run_code_analysis_path(
        &FilePath::new(dir.path().join("src").to_str().unwrap().to_string()).unwrap(),
    );
    let score = orch.calc_score(&results);

    // Clean code → score should be 100 or close
    assert!(score.value >= 90.0, "Score was {}", score.value);
}
