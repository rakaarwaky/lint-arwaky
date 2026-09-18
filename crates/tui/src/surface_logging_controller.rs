// PURPOSE: Initialize tracing-based logging for the TUI.
// Logs to `$XDG_STATE_HOME/lint-arwaky/log` (or `~/.local/state/lint-arwaky/log`)
// so the scanned project directory is never polluted. Falls back to `./log`
// when the XDG state dir is unavailable; the fallback is surfaced via eprintln
// because the TUI status line is only reachable after the event loop starts.
// No console (stdout) layer — stdout is owned by ratatui.
use shared::tui::TuiEvent;
use std::fs;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init() -> anyhow::Result<()> {
    let log_dir = match dirs::state_dir() {
        Some(state_dir) => state_dir.join("lint-arwaky").join("log"),
        None => {
            eprintln!("lint-arwaky: XDG state dir unavailable; writing TUI logs to ./log instead");
            fs::canonicalize(".")?.join("log")
        }
    };
    fs::create_dir_all(&log_dir)?;

    let file_appender = RollingFileAppender::new(Rotation::HOURLY, &log_dir, "tui.log");
    let file_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_appender);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(tracing_error::ErrorLayer::default())
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
        TuiEvent::ActionFixLive => tracing::info!(target = "tui", "ActionFixLive"),
        TuiEvent::CancelScan => tracing::info!(target = "tui", "CancelScan"),
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
        TuiEvent::ChangeProjectRoot => tracing::info!(target = "tui", "ChangeProjectRoot"),
        TuiEvent::ConfirmAction => tracing::debug!(target = "tui", "ConfirmAction"),
        TuiEvent::CancelConfirm => tracing::debug!(target = "tui", "CancelConfirm"),
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
