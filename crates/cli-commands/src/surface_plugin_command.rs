// PURPOSE: PluginCommandsSurface — CLI surface for listing adapters/plugins
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_adapters(_external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    println!("  - ESLint (JavaScript/TypeScript)");
    println!("  - Prettier (JavaScript/TypeScript)");
    println!("  - TSC (TypeScript)");
    println!("  - Ruff (Python)");
    println!("  - MyPy (Python)");
    println!("  - Bandit (Python)");
    println!("  - RustFmt (Rust)");
    println!("  - CargoAudit (Rust)");
    ExitCode::SUCCESS
}
