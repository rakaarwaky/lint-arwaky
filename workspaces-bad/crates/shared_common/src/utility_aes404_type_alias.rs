// AES404: utility defines a type alias — utility is free functions only.
pub type Helper = String;

pub fn helper(input: &str) -> Helper {
    input.to_uppercase()
}
