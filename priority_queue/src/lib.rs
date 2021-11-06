pub mod heap;
pub trait PriorityQueue {
    fn new() -> Self;
    fn from(values: &[i32]) -> Result<Box<Self>, &'static str>;
    fn insert(&mut self, value: i32) -> Result<(), &'static str>;
    fn delete_max(&mut self) -> Result<i32, &'static str>;
    fn get_max(&self) -> Result<i32, &'static str>;
}
