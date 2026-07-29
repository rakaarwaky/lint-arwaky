// BAD: Std trait in Block 2 (wrong block order)
pub struct Capabilities<NameCapability>;

impl Default for Capabilities<NameCapability> {
    fn default() -> Self {
        Self
    }
}

impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(&self, input: &<DomainVO>) {
        // protocol method should be in Block 2, not after Default
    }
}
