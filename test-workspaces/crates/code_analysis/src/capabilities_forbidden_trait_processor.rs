use crate::contract::removal_port::IRemovalPort;
use crate::taxonomy::base_vo::BaseVo;

pub struct ForbiddenTraitProcessor;
impl IRemovalPort for ForbiddenTraitProcessor {
    fn remove_background(&self, _img: Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
