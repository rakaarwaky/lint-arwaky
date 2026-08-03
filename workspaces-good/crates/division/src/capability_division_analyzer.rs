use calculator_shared::contract_calculator_protocol::CalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

pub struct DivisionAnalyzer;

impl CalculatorProtocol for DivisionAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        if expr.right == 0.0 {
            return None;
        }
        let value = expr.left / expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}
