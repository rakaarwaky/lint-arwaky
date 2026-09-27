use calculator_shared::contract_calculator_aggregate::ICalculatorAggregate;
use calculator_shared::contract_calculator_protocol::ICalculatorProtocol;
use calculator_shared::taxonomy_calculator_request::CalculatorRequest;
use calculator_shared::taxonomy_calculator_response::CalculatorResponse;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;
use calculator_shared::taxonomy_result_vo::ResultVO;
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CalculatorOrchestratorDeps {
    pub addition: Box<dyn ICalculatorProtocol>,
    pub subtraction: Box<dyn ICalculatorProtocol>,
    pub multiplication: Box<dyn ICalculatorProtocol>,
    pub division: Box<dyn ICalculatorProtocol>,
}

pub struct CalculatorOrchestrator {
    deps: CalculatorOrchestratorDeps,
    history: Mutex<Vec<ResultVO>>,
}

// ─── Block 2: Aggregate Implementation ────────────────────

impl ICalculatorAggregate for CalculatorOrchestrator {
    fn execute(&self, request: CalculatorRequest) -> CalculatorResponse {
        match request {
            CalculatorRequest::Delegate { expr } => {
                CalculatorResponse::delegation(self.delegate(&expr))
            }
            CalculatorRequest::History => CalculatorResponse::history(self.history()),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CalculatorOrchestrator {
    pub fn new(deps: CalculatorOrchestratorDeps) -> Self {
        Self {
            deps,
            history: Mutex::new(Vec::new()),
        }
    }

    fn delegate(&self, expr: &ExpressionVO) -> Option<ResultVO> {
        let analyzer = match expr.op {
            OperationVO::Add => &self.deps.addition,
            OperationVO::Subtract => &self.deps.subtraction,
            OperationVO::Multiply => &self.deps.multiplication,
            OperationVO::Divide => &self.deps.division,
        };
        let result = analyzer.evaluate(expr);
        if let Some(ref r) = result {
            if let Ok(mut log) = self.history.lock() {
                log.push(r.clone());
            }
        }
        result
    }

    fn history(&self) -> Vec<ResultVO> {
        self.history
            .lock()
            .map(|log| log.clone())
            .unwrap_or_default()
    }
}
