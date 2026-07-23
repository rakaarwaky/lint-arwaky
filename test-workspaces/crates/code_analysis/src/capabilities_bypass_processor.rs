#[allow(unused)]
pub struct BypassProcessor;

impl BypassProcessor {
    pub fn unsafe_method(&self) -> i32 {
        let x = Some(42);
        x.unwrap()
    }
}
