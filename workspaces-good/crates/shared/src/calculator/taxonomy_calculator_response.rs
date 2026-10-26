// PURPOSE: CalculatorResponse — response VOs for the calculator aggregate.

use crate::taxonomy_result_vo::ResultVO;

/// Results of a calculator aggregate request.
pub enum CalculatorResponse {
    /// Result of resolving an expression; `None` when not applicable.
    Resolved { result: Option<ResultVO> },
    /// Result of running an expression; `None` when not applicable.
    Run { result: Option<ResultVO> },
}
