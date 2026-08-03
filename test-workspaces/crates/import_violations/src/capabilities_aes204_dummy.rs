// Fixture: AES204 — dummy function to suppress unused import warning.
use taxonomy::vo::UserVO;

pub fn process() {
    let x = 42;
    println!("value: {x}");
}

fn _use_user_vo(_u: &UserVO) {}
