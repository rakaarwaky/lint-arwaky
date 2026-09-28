// AES404: utility defines a trait — utility is free functions only.
pub trait HelperContract {
    fn run(&self);
}

pub fn helper(input: &str) -> String {
    input.to_uppercase()
}
