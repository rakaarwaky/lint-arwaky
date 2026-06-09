use crate::taxonomy::base_vo::BaseVo;

pub struct ComplexViewHandler {
    pub flag: bool,
}
impl ComplexViewHandler {
    pub fn step_one(&self) {}
    pub fn step_two(&self) {}
    pub fn step_three(&self) {}
    pub fn step_four(&self) {}
    pub fn step_five(&self) {}
    pub fn step_six(&self) { if self.flag { for _ in 0..5 { let _ = true; } } }
}
