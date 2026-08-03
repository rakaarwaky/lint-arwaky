pub trait MyTrait {
    fn run(&self);
}
pub struct MyStruct;
impl MyTrait for MyStruct {
    fn run(&self) {}
}
