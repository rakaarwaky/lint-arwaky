// PURPOSE: CalculatorRequest / CalculatorResponse — aggregate request/response
// VOs for the calculator domain. The aggregate's single `execute()` entry point
// takes a CalculatorRequest variant and returns the matching CalculatorResponse
// variant; each variant carries VO-wrapped values, not raw primitives.

use crate::taxonomy_expression_vo::ExpressionVO;
use crate::taxonomy_result_vo::ResultVO;

/// Consumer verbs carried by the calculator aggregate's single entry point.
pub enum CalculatorRequest {
    /// Evaluate a single arithmetic expression and return the result.
    Delegate { expr: ExpressionVO },
    /// Return the full calculation history so far.
    History,
}

impl CalculatorRequest {
    pub fn delegate(expr: ExpressionVO) -> Self {
        Self::Delegate { expr }
    }

    pub fn history() -> Self {
        Self::History
    }
}

/// Results of a calculator aggregate request.
pub enum CalculatorResponse {
    /// Result of a single expression evaluation; `None` when the operation
    /// is not applicable (e.g. division by zero).
    Delegation { result: Option<ResultVO> },
    /// The full calculation history.
    History { results: Vec<ResultVO> },
}

impl CalculatorResponse {
    pub fn delegation(result: Option<ResultVO>) -> Self {
        Self::Delegation { result }
    }

    pub fn history(results: Vec<ResultVO>) -> Self {
        Self::History { results }
    }
}
