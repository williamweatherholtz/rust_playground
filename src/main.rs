fn main() {
    
    println!("Hello, world!");
}

trait IsPrime<T>  where T: Send{
    fn is_prime(&self) -> bool;
}
impl<T> IsPrime<T> for T where T:Send {
    fn is_prime(&self) -> bool {
        true
    }
}