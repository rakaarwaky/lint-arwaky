// Fixture: AES204 — dummy function to suppress unused import warning.
// This file triggers AES204 only, not AES203 (import is used in dummy function).
use taxonomy::entity::ProductEntity;

pub fn process() {
    let x = 42;
    println!("value: {x}");
}

fn _use_product_entity(_e: &ProductEntity) {}
