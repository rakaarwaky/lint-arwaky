// PURPOSE: calculator-domain capability contracts (AES102 `_protocol`).
//
// One file for the calculator feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::taxonomy_expression_vo::ExpressionVO;
use crate::taxonomy_result_vo::ResultVO;

pub trait ICalculatorProtocol: Send + Sync {
    /// Arithmetic evaluation capability: evaluates one expression using this
    /// capability's own operation.
    ///
    /// Evaluate a single arithmetic expression. Return the result, or `None`
    /// when the operation does not apply (e.g. division by zero).
    fn evaluate(&self, expr: &ExpressionVO) -> Option<ResultVO>;
}
