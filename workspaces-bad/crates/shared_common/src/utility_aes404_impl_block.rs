// AES404: utility defines an impl block — utility is free functions only.
struct Helper;

impl Helper {
    fn run(&self) {}
}

pub fn helper(input: &str) -> String {
    let _ = Helper;
    input.to_uppercase()
}
