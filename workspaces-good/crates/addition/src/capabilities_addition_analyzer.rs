use calculator_shared::contract_calculator_protocol::CalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;
use calculator_shared::utility_expression_parser::parse_operand;

pub struct AdditionAnalyzer;

impl AdditionAnalyzer {
    pub fn evaluate_from_str(&self, input: &str) -> Option<f64> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return None;
        }
        let left = parse_operand(parts[0])?;
        let right = parse_operand(parts[2])?;
        Some(left + right)
    }
}

impl CalculatorProtocol for AdditionAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let value = expr.left + expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}
