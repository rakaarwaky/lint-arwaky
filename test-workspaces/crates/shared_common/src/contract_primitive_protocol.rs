// AES402: Contract protocol with forbidden primitive types in method signatures
pub trait PrimitiveProtocol {
    fn get_value(&self) -> i32;
    fn set_name(&mut self, name: String);
    fn process(&self, input: u64) -> bool;
}
