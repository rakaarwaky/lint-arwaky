// AES032: Direct string and numeric literals in infrastructure
pub struct RawLiteralAdapter;

impl RawLiteralAdapter {
    pub fn process(&self) {
        let name = "raw string";
        let count = 42;
        println!("check: {}", name);
    }
}
