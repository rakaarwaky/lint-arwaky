// PURPOSE: main entry point for lint-arwaky-tui binary — bootstraps TUI container
//
// The TUI is a ratatui-based 3-panel file browser (Ranger-style):
//   - Left:   file tree (navigate directories)
//   - Center: file content preview
//   - Right:  lint results for the selected file
//
// Unlike the CLI (headless) and MCP (AI-agent), the TUI is designed for
// interactive use within the terminal. It relies on crossterm for terminal
// manipulation and ratatui for widget rendering.
use std::process::ExitCode;

fn main() -> ExitCode {
    // TuiContainer::run() initializes the terminal, enters the event loop,
    // and returns when the user quits (Ctrl+C or 'q').
    match tui::root_tui_container::TuiContainer::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("TUI error: {e}");
            ExitCode::FAILURE
        }
    }
}
