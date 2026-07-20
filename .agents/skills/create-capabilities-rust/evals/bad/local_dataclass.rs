// BAD: Data class defined in capabilities layer (AES201)
pub struct <NameResult> {
    is_valid: bool,
    reason: String,
}

pub struct Capabilities<NameCapability>;

impl Capabilities<NameCapability> {
    fn execute(&self) -> <NameResult> {
        <NameResult> { is_valid: true, reason: String::new() }
    }
}
