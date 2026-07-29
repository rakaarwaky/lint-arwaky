// BAD: No protocol trait implementation (AES403)
pub struct <NameComposer>;

impl <NameComposer> {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
