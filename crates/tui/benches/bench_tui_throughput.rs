// PURPOSE: Benchmark — measure tui component throughput.
// Layer: Benchmark (performance validation).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn bench_component_instantiation(c: &mut Criterion) {
    c.bench_function("tui_component_instantiation", |b| {
        b.iter(|| {
            let executor = Arc::new(LintExecutor::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            ));
            let handler = Arc::new(ActionHandler::new(executor));
            let _orchestrator = TuiOrchestrator::new(handler);
        });
    });
}

fn bench_event_handling(c: &mut Criterion) {
    c.bench_function("tui_event_handling", |b| {
        b.iter(|| {
            let executor = Arc::new(LintExecutor::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            ));
            let handler = Arc::new(ActionHandler::new(executor));
            let mut state = AppState::new(".".to_string());
            let event = TuiEvent::Quit;
            handler.handle(black_box(&mut state), black_box(event));
        });
    });
}

fn bench_directory_loading(c: &mut Criterion) {
    c.bench_function("tui_directory_loading", |b| {
        b.iter(|| {
            let executor = Arc::new(LintExecutor::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            ));
            let handler = Arc::new(ActionHandler::new(executor));
            let mut state = AppState::new(".".to_string());
            handler.load_directory(black_box(&mut state), "/tmp");
        });
    });
}

fn bench_preview_loading(c: &mut Criterion) {
    c.bench_function("tui_preview_loading", |b| {
        b.iter(|| {
            let executor = Arc::new(LintExecutor::new(
                code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
                    .code_analysis_linter(),
            ));
            let handler = Arc::new(ActionHandler::new(executor));
            let mut state = AppState::new(".".to_string());
            handler.load_preview(black_box(&mut state));
        });
    });
}

criterion_group!(benches, bench_component_instantiation, bench_event_handling, bench_directory_loading, bench_preview_loading);
criterion_main!(benches);
