// PURPOSE: Command: CLI surface for bootstrap — initializes tracing/subscriber logging
use std::process::ExitCode;

pub struct BootstrapCommandSurface {}

impl Default for BootstrapCommandSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl BootstrapCommandSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self) {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::ERROR.into()),
            )
            .init();
    }
}

pub fn run_cli_entry() {
    let surface = BootstrapCommandSurface::new();
    surface.execute();
}

pub fn handle_version(verbose: bool) -> ExitCode {
    let ver = env!("CARGO_PKG_VERSION");
    if verbose {
        println!("Lint Arwaky v{}", ver);
        let commit = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        println!("  Commit:    {}", commit);
        let built = option_env!("VERGEN_BUILD_TIMESTAMP").unwrap_or("runtime build");
        println!("  Built:     {}", built);
        let rustc = option_env!("VERGEN_RUSTC_SEMVER")
            .or(option_env!("RUSTC_VERSION"))
            .unwrap_or("stable");
        println!("  Rustc:     {}", rustc);
        println!("  License:   MIT");
    } else {
        crate::surface_core_command::CoreCommandsSurface::version();
    }
    ExitCode::SUCCESS
}
