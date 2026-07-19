// BAD: I/O in capabilities layer (AES404)
pub struct MyCapability;

impl MyCapability {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt").unwrap_or_default(); // FORBIDDEN
    }
}
