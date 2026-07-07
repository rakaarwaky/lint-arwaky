// PURPOSE: Initialize tracing-based logging for the TUI.
// Output: file under `log/` plus optional terminal output.
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::fs;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init() -> anyhow::Result<()> {
    let log_dir = std::path::Path::new("log");
    fs::create_dir_all(log_dir)?;

    let file_appender = RollingFileAppender::new(Rotation::HOURLY, log_dir, "tui.log");
    let file_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_appender);

    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(console_layer)
        .try_init()
        .ok();

    tracing::info!(target = "tui", "logging initialized");
    Ok(())
}

pub fn record(event: &TuiEvent) {
    match event {
        TuiEvent::MoveDown => tracing::debug!(target = "tui", "MoveDown"),
        TuiEvent::MoveUp => tracing::debug!(target = "tui", "MoveUp"),
        TuiEvent::MoveTop => tracing::debug!(target = "tui", "MoveTop"),
        TuiEvent::MoveBottom => tracing::debug!(target = "tui", "MoveBottom"),
        TuiEvent::NavigateBack => tracing::debug!(target = "tui", "NavigateBack"),
        TuiEvent::NavigateForward => tracing::debug!(target = "tui", "NavigateForward"),
        TuiEvent::FocusNext => tracing::debug!(target = "tui", "FocusNext"),
        TuiEvent::FocusPrev => tracing::debug!(target = "tui", "FocusPrev"),
        TuiEvent::ActionCheck => tracing::info!(target = "tui", "ActionCheck"),
        TuiEvent::ActionScan => tracing::info!(target = "tui", "ActionScan"),
        TuiEvent::ActionFix => tracing::info!(target = "tui", "ActionFix"),
        TuiEvent::ActionCi => tracing::info!(target = "tui", "ActionCi"),
        TuiEvent::ActionWatch => tracing::info!(target = "tui", "ActionWatch"),
        TuiEvent::ActionOrphan => tracing::info!(target = "tui", "ActionOrphan"),
        TuiEvent::ActionSecurity => tracing::info!(target = "tui", "ActionSecurity"),
        TuiEvent::ActionDuplicates => tracing::info!(target = "tui", "ActionDuplicates"),
        TuiEvent::ActionDependencies => tracing::info!(target = "tui", "ActionDependencies"),
        TuiEvent::ActionDoctor => tracing::info!(target = "tui", "ActionDoctor"),
        TuiEvent::ActionInit => tracing::info!(target = "tui", "ActionInit"),
        TuiEvent::ActionInstall => tracing::info!(target = "tui", "ActionInstall"),
        TuiEvent::ActionMcpConfig => tracing::info!(target = "tui", "ActionMcpConfig"),
        TuiEvent::ActionConfigShow => tracing::info!(target = "tui", "ActionConfigShow"),
        TuiEvent::ActionInstallHook => tracing::info!(target = "tui", "ActionInstallHook"),
        TuiEvent::ActionUninstallHook => tracing::info!(target = "tui", "ActionUninstallHook"),
        TuiEvent::ActionAdapters => tracing::info!(target = "tui", "ActionAdapters"),
        TuiEvent::ActionVersion => tracing::info!(target = "tui", "ActionVersion"),
        TuiEvent::ToggleHelp => tracing::debug!(target = "tui", "ToggleHelp"),
        TuiEvent::ToggleSearch => tracing::debug!(target = "tui", "ToggleSearch"),
        TuiEvent::SearchInput(c) => tracing::debug!(target = "tui", "SearchInput({})", c),
        TuiEvent::SearchBackspace => tracing::debug!(target = "tui", "SearchBackspace"),
        TuiEvent::SearchConfirm => tracing::debug!(target = "tui", "SearchConfirm"),
        TuiEvent::SearchCancel => tracing::debug!(target = "tui", "SearchCancel"),
        TuiEvent::PathInput(c) => tracing::debug!(target = "tui", "PathInput({})", c),
        TuiEvent::PathBackspace => tracing::debug!(target = "tui", "PathBackspace"),
        TuiEvent::PathConfirm => tracing::debug!(target = "tui", "PathConfirm"),
        TuiEvent::PathUseCurrent => tracing::debug!(target = "tui", "PathUseCurrent"),
        TuiEvent::Quit => tracing::info!(target = "tui", "Quit"),
        TuiEvent::Resize(w, h) => tracing::debug!(target = "tui", "Resize({},{})", w, h),
        TuiEvent::MouseClick(col, row) => {
            tracing::debug!(target = "tui", "MouseClick({},{})", col, row)
        }
        TuiEvent::MouseDrag(col, row) => {
            tracing::debug!(target = "tui", "MouseDrag({},{})", col, row)
        }
        TuiEvent::MouseScrollUp(col, row) => {
            tracing::debug!(target = "tui", "MouseScrollUp({},{})", col, row)
        }
        TuiEvent::MouseScrollDown(col, row) => {
            tracing::debug!(target = "tui", "MouseScrollDown({},{})", col, row)
        }
        TuiEvent::CopyToClipboard => tracing::info!(target = "tui", "CopyToClipboard"),
        TuiEvent::CopyToFile => tracing::info!(target = "tui", "CopyToFile"),
        TuiEvent::PreviewScrollUp => tracing::debug!(target = "tui", "PreviewScrollUp"),
        TuiEvent::PreviewScrollDown => tracing::debug!(target = "tui", "PreviewScrollDown"),
        TuiEvent::Tick => {}
        TuiEvent::None => {}
    }
}
