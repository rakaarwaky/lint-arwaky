use crate::taxonomy::base_vo::BaseVo;

pub trait WrongNamePort {
    fn execute(&self, input: String) -> bool;
}
pub struct WrongNameImpl;
impl WrongNamePort for WrongNameImpl {
    fn execute(&self, input: String) -> bool { true }
}
