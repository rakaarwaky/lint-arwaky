// AES038: Missing VO - method call missing required VO parameter
pub struct CapabilitiesMissingVoValidator;

impl CapabilitiesMissingVoValidator {
    pub fn validate(&self, raw_id: i32) -> bool {
        raw_id > 0
    }
}
