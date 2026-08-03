// AES304: bypass annotation - unwrap() and expect()
pub struct BypassEntity;

impl BypassEntity {
    pub fn unsafe_method(&self) -> &str {
        let s = Some("hello");
        let _ = s.unwrap();
        let _ = format!("test").parse::<i32>().expect("must parse");
        "done"
    }
}
