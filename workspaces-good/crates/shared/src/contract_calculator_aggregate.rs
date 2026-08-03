use crate::taxonomy_expression_vo::ExpressionVO;
use crate::taxonomy_result_vo::ResultVO;

pub trait CalculatorAggregate {
    fn delegate(&mut self, expr: &ExpressionVO) -> Option<ResultVO>;
    fn history(&self) -> Vec<ResultVO>;
}
