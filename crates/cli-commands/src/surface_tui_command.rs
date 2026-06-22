// PURPOSE: TuiCommandSurface — interactive menu-driven TUI for lint-arwaky-tui binary
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::process::ExitCode;

pub struct TuiCommandSurface;

impl TuiCommandSurface {
    pub fn run() -> ExitCode {
        run_tui_loop()
    }
}

fn cli_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-cli");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }
    "lint-arwaky-cli".to_string()
}

fn print_header(term: &Term) {
    let _ = term.clear_screen();
    println!(
        "{}",
        style("╔══════════════════════════════════════════════════╗")
            .cyan()
            .bold()
    );
    println!(
        "{}  {}  {}",
        style("║").cyan().bold(),
        style("  Lint Arwaky TUI  -- Code Quality Gateway")
            .white()
            .bold(),
        style("║").cyan().bold()
    );
    println!(
        "{}",
        style("╚══════════════════════════════════════════════════╝")
            .cyan()
            .bold()
    );
    println!();
}

fn ask_path(prompt: &str, default: &str) -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .unwrap_or_else(|_| default.to_string())
}

fn run_cmd(args: &[&str]) {
    let cli = cli_binary();
    println!(
        "\n{} {} {}\n",
        style(">").green().bold(),
        style("Running:").dim(),
        style(format!("{} {}", cli, args.join(" "))).yellow()
    );
    let status = std::process::Command::new(&cli).args(args).status();
    match status {
        Ok(s) if s.success() => {
            println!(
                "\n{} {}",
                style("OK").green().bold(),
                style("Done.").green()
            )
        }
        Ok(s) => println!(
            "\n{} Exit code: {}",
            style("FAIL").red().bold(),
            s.code().unwrap_or(-1)
        ),
        Err(e) => println!("\n{} Failed to run binary: {e}", style("FAIL").red().bold()),
    }
}

fn pause() {
    let _ = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(style("Press Enter to return to menu").dim().to_string())
        .default(String::new())
        .allow_empty(true)
        .interact_text();
}

#[derive(Clone, Copy, PartialEq)]
enum MenuKind {
    Action,
    Separator,
}

struct MenuItem {
    label: &'static str,
    id: &'static str,
    kind: MenuKind,
}

const MENU: &[MenuItem] = &[
    MenuItem {
        label: "[check]   AES self-lint audit",
        id: "check",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[scan]    Full multi-adapter scan",
        id: "scan",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[fix]     Apply safe automatic fixes",
        id: "fix",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[ci]      CI mode (exit 1 if score < N)",
        id: "ci",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[orphan]        Check orphan files (AES501-506)",
        id: "orphan",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[security]      Vulnerability scan",
        id: "security",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[duplicates]    Duplication detection",
        id: "duplicates",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[dependencies]  Library vulnerability scan",
        id: "dependencies",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[watch]  Watch and lint on changes",
        id: "watch",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[doctor]      Diagnose environment",
        id: "doctor",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[init]        Create default config",
        id: "init",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[install]     Install adapter deps",
        id: "install",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[mcp-config]  Print MCP config",
        id: "mcp-config",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[config-show] Show active config",
        id: "config-show",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[install-hook]   Install git pre-commit",
        id: "install-hook",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[uninstall-hook] Remove git pre-commit",
        id: "uninstall-hook",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "[adapters]  List active adapters",
        id: "adapters",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "[version]   Show version",
        id: "version",
        kind: MenuKind::Action,
    },
    MenuItem {
        label: "",
        id: "",
        kind: MenuKind::Separator,
    },
    MenuItem {
        label: "Exit",
        id: "exit",
        kind: MenuKind::Action,
    },
];

pub fn run_tui_loop() -> ExitCode {
    let term = Term::stdout();

    loop {
        print_header(&term);

        let selectable: Vec<(usize, &MenuItem)> = MENU
            .iter()
            .enumerate()
            .filter(|(_, m)| m.kind == MenuKind::Action)
            .collect();

        let display_labels: Vec<&str> = selectable.iter().map(|(_, m)| m.label).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select command")
            .items(&display_labels)
            .default(0)
            .interact_on_opt(&term);

        let pick = match selection {
            Ok(Some(i)) => i,
            Ok(None) | Err(_) => break,
        };

        let item = selectable[pick].1;
        println!();

        match item.id {
            "exit" => break,
            "check" => {
                let p = ask_path("Path", ".");
                run_cmd(&["check", &p]);
                pause();
            }
            "scan" => {
                let p = ask_path("Path", ".");
                run_cmd(&["scan", &p]);
                pause();
            }
            "fix" => {
                let p = ask_path("Path", ".");
                run_cmd(&["fix", &p]);
                pause();
            }
            "ci" => {
                let p = ask_path("Path", ".");
                let t: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Threshold")
                    .default("80".to_string())
                    .interact_text()
                    .unwrap_or_else(|_| "80".to_string());
                run_cmd(&["ci", &p, "--threshold", &t]);
                pause();
            }
            "orphan" => {
                let p = ask_path("Path", ".");
                run_cmd(&["orphan", &p]);
                pause();
            }
            "security" => {
                let p = ask_path("Path", ".");
                run_cmd(&["security", &p]);
                pause();
            }
            "duplicates" => {
                let p = ask_path("Path", ".");
                run_cmd(&["duplicates", &p]);
                pause();
            }
            "dependencies" => {
                let p = ask_path("Path", ".");
                run_cmd(&["dependencies", &p]);
                pause();
            }
            "watch" => {
                let p = ask_path("Path", ".");
                run_cmd(&["watch", &p]);
                pause();
            }
            "doctor" => {
                run_cmd(&["doctor"]);
                pause();
            }
            "init" => {
                run_cmd(&["init"]);
                pause();
            }
            "install" => {
                run_cmd(&["install"]);
                pause();
            }
            "mcp-config" => {
                run_cmd(&["mcp-config"]);
                pause();
            }
            "config-show" => {
                run_cmd(&["config-show"]);
                pause();
            }
            "install-hook" => {
                run_cmd(&["install-hook"]);
                pause();
            }
            "uninstall-hook" => {
                run_cmd(&["uninstall-hook"]);
                pause();
            }
            "adapters" => {
                run_cmd(&["adapters"]);
                pause();
            }
            "version" => {
                run_cmd(&["version"]);
                pause();
            }
            _ => {}
        }
    }

    println!("\n{}", style("Bye!").cyan().bold());
    ExitCode::SUCCESS
}
