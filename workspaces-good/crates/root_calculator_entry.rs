use clap::{Parser, Subcommand};

use calculator_addition::capabilities_addition_analyzer::AdditionAnalyzer;
use calculator_cli::surface_calculator_command;
use calculator_division::capabilities_division_analyzer::DivisionAnalyzer;
use calculator_multiplication::capabilities_multiplication_analyzer::MultiplicationAnalyzer;
use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;
use calculator_shared::contract_calculator_protocol::CalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;
use calculator_shared::taxonomy_result_vo::ResultVO;
use calculator_subtraction::capabilities_subtraction_analyzer::SubtractionAnalyzer;

#[derive(Parser)]
#[command(name = "calculator", version, about = "Interactive calculator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the interactive calculator
    Run {
        /// Optional working path
        #[arg(long)]
        path: Option<String>,
    },
}

struct Calculator {
    history: Vec<ResultVO>,
}

impl Calculator {
    fn new() -> Self {
        Self { history: Vec::new() }
    }
}

impl CalculatorAggregate for Calculator {
    fn delegate(&mut self, expr: &ExpressionVO) -> Option<ResultVO> {
        let result = match expr.op {
            OperationVO::Add => AdditionAnalyzer.evaluate(expr),
            OperationVO::Subtract => SubtractionAnalyzer.evaluate(expr),
            OperationVO::Multiply => MultiplicationAnalyzer.evaluate(expr),
            OperationVO::Divide => DivisionAnalyzer.evaluate(expr),
        };
        if let Some(ref r) = result {
            self.history.push(r.clone());
        }
        result
    }

    fn history(&self) -> Vec<ResultVO> {
        self.history.clone()
    }
}

fn main() {
    let cli = Cli::parse();
    let mut calc = Calculator::new();

    match cli.command {
        Some(Command::Run { path: _ }) | None => {
            surface_calculator_command::run(&mut calc);
        }
    }
}
