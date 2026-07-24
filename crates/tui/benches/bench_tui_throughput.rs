// PURPOSE: Benchmark — measure tui component throughput.
// Layer: Benchmark (performance validation).
// Best practices: significance_level(0.05), sample_size(30+), avoid instantiation inside iter

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_tui_stack() -> (Arc<LintExecutor>, Arc<ActionHandler>) {
    let executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let handler = Arc::new(ActionHandler::new(executor));
    (executor, handler)
}

fn bench_component_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_instantiation");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    let (executor, handler) = build_tui_stack();
                    let _orchestrator = TuiOrchestrator::new(handler);
                    black_box((_orchestrator, executor));
                }
            });
        });
    }
    group.finish();
}

fn bench_event_handling(c: &mut Criterion) {
    let (executor, handler) = build_tui_stack();
    let mut group = c.benchmark_group("event_handling");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::new("events", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                let mut state = AppState::new(".".to_string());
                let event = TuiEvent::Quit;
                for _ in 0..count {
                    black_box(handler.handle(&mut state, event));
                }
            });
        });
    }
    group.finish();
}

fn bench_directory_loading(c: &mut Criterion) {
    let (_, handler) = build_tui_stack();
    let mut group = c.benchmark_group("directory_loading");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::new("loads", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                let mut state = AppState::new(".".to_string());
                for _ in 0..count {
                    black_box(handler.load_directory(&mut state, "/tmp"));
                }
            });
        });
    }
    group.finish();
}

fn bench_preview_loading(c: &mut Criterion) {
    let (_, handler) = build_tui_stack();
    let mut group = c.benchmark_group("preview_loading");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.bench_with_input(BenchmarkId::new("loads", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                let mut state = AppState::new(".".to_string());
                for _ in 0..count {
                    black_box(handler.load_preview(&mut state));
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_component_instantiation,
    bench_event_handling,
    bench_directory_loading,
    bench_preview_loading
);
criterion_main!(benches);
