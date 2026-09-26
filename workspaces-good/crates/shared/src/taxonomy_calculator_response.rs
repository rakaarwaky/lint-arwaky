// PURPOSE: CalculatorResponse — response VOs for the calculator aggregate.

use crate::taxonomy_result_vo::ResultVO;

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
