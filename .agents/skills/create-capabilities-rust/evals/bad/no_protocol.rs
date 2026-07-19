// BAD: No protocol trait implementation (AES403)
pub struct FrameComposer;

impl FrameComposer {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
