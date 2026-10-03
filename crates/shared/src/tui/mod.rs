// tui — shared TUI foundation: VO, contract, and utility types shared
// across lint-arwaky. Split out of `tui-lint-arwaky`.
pub mod taxonomy_lint_execution_vo;
pub mod utility_file_system;
pub mod utility_report_formatter;
pub mod utility_tui_theme;

pub use taxonomy_lint_execution_vo::{LintExecutionResult, LintOutcome, outcome_label};
