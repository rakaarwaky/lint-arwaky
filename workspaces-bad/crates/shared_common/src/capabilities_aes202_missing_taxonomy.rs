// AES202: capabilities missing mandatory taxonomy import
use shared::contract::contract_test_fixture_protocol::TestProtocol;

pub struct MandatoryMissingChecker;

impl MandatoryMissingChecker {
    pub fn validate(&self, proto: &dyn TestProtocol) -> bool {
        proto.validate()
    }
}
