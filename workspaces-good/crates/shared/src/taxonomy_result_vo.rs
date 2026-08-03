use super::taxonomy_operation_vo::OperationVO;

#[derive(Debug, Clone)]
pub struct ResultVO {
    pub expression: String,
    pub value: f64,
}

impl ResultVO {
    pub fn new(left: f64, op: &OperationVO, right: f64, value: f64) -> Self {
        Self {
            expression: format!("{} {} {} = {}", left, op.symbol(), right, value),
            value,
        }
    }
}
