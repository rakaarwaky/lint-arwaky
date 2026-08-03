use super::taxonomy_operation_vo::OperationVO;

pub struct ExpressionVO {
    pub left: f64,
    pub op: OperationVO,
    pub right: f64,
}

impl ExpressionVO {
    pub fn new(left: f64, op: OperationVO, right: f64) -> Self {
        Self { left, op, right }
    }
}
