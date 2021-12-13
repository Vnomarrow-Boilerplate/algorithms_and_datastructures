// examples/vector.rs
extern crate data_structures;
use data_structures::*;
fn main() {
    let mut v1: Vec2D<i32> = Vec2D::new(10, 10, 0);
    *v1.get_mut(0, 0).unwrap() = 10;
    *v1.get_mut(0, 5).unwrap() = 15;
    println!("{}", *v1.get_mut(0, 0).unwrap());


    let mut v2: Vec2DShifted<i64> = Vec2DShifted::new(10, 10, 10, 10, 0);
    *v2.get_mut(15, 12).unwrap() = 10;
    *v2.get_mut(17, 15).unwrap() = 15;
    println!("{}", *v2.get(15, 12).unwrap());

}