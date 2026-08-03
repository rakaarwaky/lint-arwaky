use crate::taxonomy::base_vo::BaseVo;

pub struct ForbiddenTraitProcessor;
impl crate::contract::some_protocol::SomeProtocol for ForbiddenTraitProcessor {
    fn required_fn(&self) -> bool {
        true
    }
}
