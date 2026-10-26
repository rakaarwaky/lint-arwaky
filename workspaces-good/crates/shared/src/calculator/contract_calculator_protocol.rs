// PURPOSE: calculator-domain capability contracts (AES102 `_protocol`).
//
// Two FR requirements (FR-CALCULATOR-001, FR-CALCULATOR-002) map to two
// protocol classes here. Methods mirror every method promised in the
// calculator FRD API Contract tables across crates/, modules/, and
// packages/.

use crate::taxonomy_expression_vo::ExpressionVO;
use crate::taxonomy_result_vo::ResultVO;

/// Resolve a single arithmetic expression (FR-CALCULATOR-001).
pub trait IResolveProtocol: Send + Sync {
    fn resolve(&self, expr: &ExpressionVO) -> Option<ResultVO>;
    /// Route a request to the feature that owns the operation.
    fn route(&self, expr: &ExpressionVO) -> Option<ResultVO>;
}

/// Run a single arithmetic expression and return the result (FR-CALCULATOR-002).
pub trait IRunProtocol: Send + Sync {
    fn run(&self, expr: &ExpressionVO) -> Option<ResultVO>;
    /// Merge the operation logs newest first.
    fn merge(&self, expr: &ExpressionVO) -> Option<ResultVO>;
}
