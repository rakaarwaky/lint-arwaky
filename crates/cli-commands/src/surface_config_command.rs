// PURPOSE: handle_config — CLI surface for displaying current lint-arwaky configuration from YAML files
use std::process::ExitCode;

use crate::surface_core_command::ConfigCommands;

pub fn handle_config(command: ConfigCommands) -> ExitCode {
    match command {
        ConfigCommands::Show => {
            let languages = ["rust", "python", "javascript"];
            let mut found = false;

            for lang in languages {
                let filename = format!("lint_arwaky.config.{}.yaml", lang);

                // Priority 1: CWD
                let cwd_path = std::path::PathBuf::from(&filename);
                // Priority 2: XDG user config (~/.config/lint-arwaky/)
                let xdg_user_path =
                    dirs::config_dir().map(|d| d.join("lint-arwaky").join(&filename));
                // Priority 3: XDG system config (/etc/xdg/lint-arwaky/)
                let xdg_system_path = {
                    let system_dirs =
                        std::env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_string());
                    system_dirs.split(':').find_map(|dir| {
                        let p = std::path::PathBuf::from(dir)
                            .join("lint-arwaky")
                            .join(&filename);
                        if p.exists() {
                            Some(p)
                        } else {
                            None
                        }
                    })
                };

                let candidates = [
                    (cwd_path, "CWD"),
                    (
                        xdg_user_path.unwrap_or_default(),
                        "XDG user (~/.config/lint-arwaky)",
                    ),
                    (
                        xdg_system_path.unwrap_or_default(),
                        "XDG system (/etc/xdg/lint-arwaky)",
                    ),
                ];

                for (path, source) in candidates {
                    if path.exists() {
                        println!("Config source: {} ({})", path.display(), source);
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            println!("{}", content);
                        }
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                println!("Config source: built-in defaults");
                println!("No lint_arwaky.config.*.yaml found in:");
                println!("  - CWD");
                println!("  - ~/.config/lint-arwaky/");
                println!("  - /etc/xdg/lint-arwaky/");
                println!("Run `setup init` to create one in ~/.config/lint-arwaky/.");
            }
        }
    }
    ExitCode::SUCCESS
}
