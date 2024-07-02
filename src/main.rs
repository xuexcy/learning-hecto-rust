use std::fmt::{Debug, Result, Display, Formatter};
struct Hecto;

impl Debug for Hecto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "hecto is awesome.");
    }
}
impl Display for Hecto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "hector is displaying now");
    }
}
fn main() {
    let x = Hecto{};
    println!("x is {:?}",x); // This would prints out "x is Hecto"
    println!("x is {}",x); // This would prints out "x is Hecto"
    println!("{0:?}, {0}",x); // This would prints out "x is Hecto"
    //let y = x;
    //println!("x is {:?}, and y is {:?}",x, y); // ... and this?

    let slice: &str = "hello, world";
    dbg!(slice.as_ptr());
    dbg!(slice.len());
    dbg!(slice.as_bytes());
}
