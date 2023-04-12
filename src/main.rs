pub mod vector;
use crate::vector::Vector3;

fn main() {
    vector3_example();
}

fn vector3_example() {
    let u = Vector3::new(1.0, 2.0, 3.0);
    let v = Vector3::new(1.0, 2.0, 3.0);
    let add = &u + &v;
    let sub = &u - &v;

    println!("add: {:?}", add);
    println!("sub: {:?}", sub);
}
