// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use std::process::ExitCode;

pub fn handle_config_show() -> ExitCode {
    let config_paths = [
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ];

    for path in &config_paths {
        if std::path::Path::new(path).exists() {
            println!("Found: {path}");
            match std::fs::read_to_string(path) {
                Ok(content) => println!("{content}"),
                Err(e) => println!("Error reading {path}: {e}"),
            }
            return ExitCode::SUCCESS;
        }
    }

    println!("No config file found. Run `lint-arwaky init` to create one.");
    ExitCode::SUCCESS
}
