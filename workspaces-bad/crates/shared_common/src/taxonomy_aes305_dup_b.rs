pub struct DupEntityA;

impl DupEntityA {
    pub fn do_something(&self) -> String {
        let x = 1;
        let y = 2;
        let z = x + y;
        format!("result: {}", z)
    }
}
