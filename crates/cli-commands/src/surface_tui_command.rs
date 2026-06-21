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

// ── Helpers ──────────────────────────────────────────────────────────────────

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
            println!("\n{} {}", style("OK").green().bold(), style("Done.").green())
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

// ── Menu definition ───────────────────────────────────────────────────────────

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
    // -- Core commands --
    MenuItem { label: "[check]   AES self-lint audit", id: "check", kind: MenuKind::Action },
    MenuItem { label: "[scan]    Full multi-adapter scan", id: "scan", kind: MenuKind::Action },
    MenuItem { label: "[fix]     Apply safe automatic fixes", id: "fix", kind: MenuKind::Action },
    MenuItem { label: "[ci]      CI mode (exit 1 if score < N)", id: "ci", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    // -- Detection commands --
    MenuItem { label: "[orphan]        Check orphan files (AES501-506)", id: "orphan", kind: MenuKind::Action },
    MenuItem { label: "[security]      Vulnerability scan", id: "security", kind: MenuKind::Action },
    MenuItem { label: "[duplicates]    Duplication detection", id: "duplicates", kind: MenuKind::Action },
    MenuItem { label: "[dependencies]  Library vulnerability scan", id: "dependencies", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    // -- Watch --
    MenuItem { label: "[watch]  Watch and lint on changes", id: "watch", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    // -- Setup & config --
    MenuItem { label: "[setup install]    Install all adapters", id: "setup-install", kind: MenuKind::Action },
    MenuItem { label: "[setup init]       Create default config", id: "setup-init", kind: MenuKind::Action },
    MenuItem { label: "[config show]      Show active configuration", id: "config-show", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    // -- Git --
    MenuItem { label: "[install-hook]     Install git pre-commit hook", id: "install-hook", kind: MenuKind::Action },
    MenuItem { label: "[uninstall-hook]   Remove git pre-commit hook", id: "uninstall-hook", kind: MenuKind::Action },
    MenuItem { label: "[git-diff]         Show files changed since base", id: "git-diff", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    // -- Info --
    MenuItem { label: "[maintenance doctor]  Diagnose environment", id: "maintenance-doctor", kind: MenuKind::Action },
    MenuItem { label: "[version]      Show version", id: "version", kind: MenuKind::Action },
    MenuItem { label: "[adapters]     List active adapters", id: "adapters", kind: MenuKind::Action },
    MenuItem { label: "[vscode-graph] Export graph JSON for VS Code", id: "vscode-graph", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "Exit", id: "exit", kind: MenuKind::Action },
];

// ── Main loop ─────────────────────────────────────────────────────────────────

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
                let path = ask_path("Path to check", ".");
                run_cmd(&["check", &path]);
                pause();
            }

            "scan" => {
                let path = ask_path("Path to scan", ".");
                run_cmd(&["scan", &path]);
                pause();
            }

            "fix" => {
                let path = ask_path("Path to fix", ".");
                run_cmd(&["fix", &path]);
                pause();
            }

            "ci" => {
                let path = ask_path("Path", ".");
                let threshold: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Minimum score threshold")
                    .default("80".to_string())
                    .interact_text()
                    .unwrap_or_else(|_| "80".to_string());
                run_cmd(&["ci", &path, "--threshold", &threshold]);
                pause();
            }

            "orphan" => {
                let path = ask_path("File path to check", ".");
                run_cmd(&["orphan", &path]);
                pause();
            }

            "security" => {
                let path = ask_path("Path to scan", ".");
                run_cmd(&["security", &path]);
                pause();
            }

            "duplicates" => {
                let path = ask_path("Path to analyze", ".");
                run_cmd(&["duplicates", &path]);
                pause();
            }

            "dependencies" => {
                let path = ask_path("Path to scan", ".");
                run_cmd(&["dependencies", &path]);
                pause();
            }

            "watch" => {
                let path = ask_path("Path to watch", ".");
                run_cmd(&["watch", &path]);
                pause();
            }

            "setup-install" => {
                run_cmd(&["setup", "install"]);
                pause();
            }

            "setup-init" => {
                run_cmd(&["setup", "init"]);
                pause();
            }

            "config-show" => {
                run_cmd(&["config", "show"]);
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

            "git-diff" => {
                run_cmd(&["git-diff"]);
                pause();
            }

            "maintenance-doctor" => {
                run_cmd(&["maintenance", "doctor"]);
                pause();
            }

            "version" => {
                run_cmd(&["version"]);
                pause();
            }

            "adapters" => {
                run_cmd(&["adapters"]);
                pause();
            }

            "vscode-graph" => {
                run_cmd(&["vscode-graph"]);
                pause();
            }

            _ => {
                eprintln!("Warning: unhandled case {:?} in {}", item.id, module_path!());
            }
        }
    }

    println!("\n{}", style("Bye!").cyan().bold());
    ExitCode::SUCCESS
}
