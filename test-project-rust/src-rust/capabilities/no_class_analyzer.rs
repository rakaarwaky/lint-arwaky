// This capability file violates AES009 (mandatory-class-definition)
// because it contains only free-standing helper functions and lacks any struct, enum, or trait definition.
use crate::taxonomy::removal_types::RemovalType;

pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply_two_numbers(a: i32, b: i32) -> i32 {
    a * b
}
