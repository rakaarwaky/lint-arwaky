use clap::{Parser, Subcommand};
use calculator::root_calculator_container::CalculatorContainer;
use calculator_cli::surface_calculator_command;

#[derive(Parser)]
#[command(name = "calculator", version, about = "Interactive calculator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut container = CalculatorContainer::new();

    match cli.command {
        Some(Command::Run { path: _ }) | None => {
            surface_calculator_command::run(container.orchestrator());
        }
    }
}

