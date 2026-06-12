// PURPOSE: handle_config — CLI surface for displaying current lint-arwaky configuration from YAML files
use std::process::ExitCode;

use crate::surface_core_command::ConfigCommands;

pub fn handle_config(command: ConfigCommands) -> ExitCode {
    match command {
        ConfigCommands::Show => {
            let config_paths = [
                "lint_arwaky.config.rust.yaml",
                "lint_arwaky.config.python.yaml",
                "lint_arwaky.config.javascript.yaml",
            ];
            let mut found = false;
            for cp in &config_paths {
                let path = std::path::Path::new(cp);
                if path.exists() {
                    println!("Config source: {} (loaded)", cp);
                    if let Ok(content) = std::fs::read_to_string(cp) {
                        println!("{}", content);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                println!("Config source: built-in defaults");
                println!("No lint_arwaky.config.*.yaml found.");
                println!("Run `setup init` to create one.");
            }
        }
    }
    ExitCode::SUCCESS
}
