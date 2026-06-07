// AES026: #[derive(Clone, Debug)] uses forbidden derives
#[derive(Clone, Debug)]
pub struct DerivedBadInfra;

impl DerivedBadInfra {
    pub fn run(&self) {}
}
