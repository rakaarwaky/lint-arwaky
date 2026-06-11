// PURPOSE: main entry point for lint-arwaky-tui — interactive TUI launcher
use std::process::ExitCode;

use lint_arwaky::cli_commands::surface_tui_command::TuiCommandSurface;

pub struct TuiMainEntry {}

fn main() -> ExitCode {
    TuiCommandSurface::run()
}
