// PURPOSE: main entry point for lint-arwaky-tui binary — bootstraps TUI container
use std::process::ExitCode;

fn main() -> ExitCode {
    match tui::root_tui_container::TuiContainer::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("TUI error: {e}");
            ExitCode::FAILURE
        }
    }
}
