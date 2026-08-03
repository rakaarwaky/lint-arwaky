use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;
use calculator_shared::contract_calculator_protocol::CalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

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
            OperationVO::Add => calculator_addition::capability_addition_analyzer::AdditionAnalyzer
                .evaluate(expr),
            OperationVO::Subtract => {
                calculator_subtraction::capability_subtraction_analyzer::SubtractionAnalyzer
                    .evaluate(expr)
            }
            OperationVO::Multiply => {
                calculator_multiplication::capability_multiplication_analyzer::MultiplicationAnalyzer
                    .evaluate(expr)
            }
            OperationVO::Divide => {
                calculator_division::capability_division_analyzer::DivisionAnalyzer.evaluate(expr)
            }
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
    let mut calc = Calculator::new();
    calculator_cli::surface_calculator_command::run(&mut calc);
}
