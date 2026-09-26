use calculator_shared::contract_calculator_protocol::ICalculatorProtocol;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_result_vo::ResultVO;

pub struct MultiplicationAnalyzer;

impl ICalculatorProtocol for MultiplicationAnalyzer {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let value = expr.left * expr.right;
        Some(ResultVO::new(expr.left, &expr.op, expr.right, value))
    }
}
