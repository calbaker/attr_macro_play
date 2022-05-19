extern crate proc_macros;
use proc_macros::api;

#[api]
pub struct MyStruct {
    pub x: i32,
    // #[api(skip)]
    pub y: i32,
}

fn main() {
    let s = MyStruct { x: 5, y: 10 };
    println!("x is {}", s.get_x());
    println!("y is {}", s.get_y());
}
