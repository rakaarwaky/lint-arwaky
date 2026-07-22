// PURPOSE: Benchmark tests — performance regression for auto-fix operations.
// Uses criterion for statistically sound measurements.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_common_vo::{LineNumber, Score};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use std::io::Write;
use std::sync::Arc;

// ─── Mock linter for benchmarks ───────────────────────────

struct BenchLinter { results: Vec<LintResult> }
impl ICodeAnalysisAggregate for BenchLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList { LintResultList::new(self.results.clone()) }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList { LintResultList::new(self.results.clone()) }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> { self.results.clone() }
    fn calc_score(&self, _: &[LintResult]) -> Score { Score::new(100.0) }
    fn check_critical(&self, _: &[LintResult]) -> bool { false }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> String { String::new() }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> { vec![] }
}

// ─── Benchmarks ───────────────────────────────────────────

fn bench_fix_unused_import(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_unused_import");

    for line_count in [10, 100, 1000] {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        for i in 0..line_count {
            if i == 0 {
                writeln!(tmp, "use std::io;").unwrap();
            } else {
                writeln!(tmp, "fn func_{}() {{}}", i).unwrap();
            }
        }
        tmp.flush().unwrap();

        let file_path = tmp.path().to_str().unwrap().to_string();
        let sut = LintFixProcessor::new(Arc::new(BenchLinter { results: vec![] }));

        group.bench_with_input(
            BenchmarkId::new("remove_line_1", line_count),
            &line_count,
            |b, _| {
                b.iter(|| {
                    // Re-create file for each iteration to avoid idempotency short-circuit
                    let mut f = tempfile::NamedTempFile::new().unwrap();
                    for i in 0..line_count {
                        if i == 0 {
                            writeln!(f, "use std::io;").unwrap();
                        } else {
                            writeln!(f, "fn func_{}() {{}}", i).unwrap();
                        }
                    }
                    f.flush().unwrap();
                    let fp = f.path().to_str().unwrap().to_string();
                    let s = LintFixProcessor::new(Arc::new(BenchLinter { results: vec![] }));
                    s.fix_unused_import(&fp, LineNumber::new(1))
                });
            },
        );
    }
    group.finish();
}

fn bench_fix_bypass_comments(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_bypass_comments");

    for line_count in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("remove_allow_attr", line_count),
            &line_count,
            |b, &lc| {
                b.iter(|| {
                    let mut f = tempfile::NamedTempFile::new().unwrap();
                    writeln!(f, "#[allow(unused)]").unwrap();
                    for i in 1..lc {
                        writeln!(f, "fn func_{}() {{}}", i).unwrap();
                    }
                    f.flush().unwrap();
                    let fp = f.path().to_str().unwrap().to_string();
                    let s = LintFixProcessor::new(Arc::new(BenchLinter { results: vec![] }));
                    s.fix_bypass_comments(&fp, LineNumber::new(1))
                });
            },
        );
    }
    group.finish();
}

fn bench_file_adapter_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_adapter_read");

    for size_kb in [1, 10, 100] {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        let content = "x".repeat(size_kb * 1024);
        tmp.write_all(content.as_bytes()).unwrap();
        tmp.flush().unwrap();

        let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();
        let adapter = FileAdapter::new();

        group.bench_with_input(
            BenchmarkId::new("read_file", format!("{}kb", size_kb)),
            &path,
            |b, p| {
                b.iter(|| adapter.read_file(p));
            },
        );
    }
    group.finish();
}

fn bench_execute_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("execute_pipeline");

    for violation_count in [0, 5, 20] {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        for i in 0..50 {
            writeln!(tmp, "fn func_{}() {{}}", i).unwrap();
        }
        tmp.flush().unwrap();

        let file_path = tmp.path().to_str().unwrap().to_string();
        let violations: Vec<LintResult> = (0..violation_count)
            .map(|i| {
                LintResult::new_arch(
                    &file_path,
                    (i + 1) as usize,
                    "AES203",
                    Severity::Warning,
                    format!("unused import {}", i),
                )
            })
            .collect();

        let linter = BenchLinter { results: violations };
        let sut = LintFixProcessor::new(Arc::new(linter));
        let path = FilePath::new(file_path.clone()).unwrap();

        group.bench_with_input(
            BenchmarkId::new("execute", violation_count),
            &violation_count,
            |b, _| {
                b.iter(|| sut.execute(&path));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_fix_unused_import,
    bench_fix_bypass_comments,
    bench_file_adapter_read,
    bench_execute_pipeline,
);
criterion_main!(benches);
