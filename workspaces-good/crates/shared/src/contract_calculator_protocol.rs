use crate::taxonomy_expression_vo::ExpressionVO;
use crate::taxonomy_result_vo::ResultVO;

pub trait CalculatorProtocol {
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO>;
}
