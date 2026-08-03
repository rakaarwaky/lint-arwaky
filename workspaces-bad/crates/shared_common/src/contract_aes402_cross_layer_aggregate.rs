// AES402: contract uses primitive types instead of taxonomy VOs
pub trait ConfigProtocol {
    fn load(&self, id: i32, name: &str) -> bool;
    fn save(&self, data: Vec<u8>) -> Result<(), String>;
}
