use std::f32::consts::PI;

use glam::{Mat3, Vec3};

fn main() {
    // Basic Example of rust
    println!("Example of rust");
    let (a, b) = (1.0f32, 2.0f32);
    println!("{} + {} = {}", a, b, a + b);
    println!("{}", b.sqrt());
    println!("{}", a.acos());
    println!("{}", (PI / 6.0) * a.acos());

    // Example of vector
    println!("Example of vector");
    let v = Vec3::new(1.0, 2.0, 3.0);
    let w = Vec3::new(4.0, 5.0, 6.0);
    // vector output
    println!("Example of output");
    println!("{}", v);
    // vector add
    println!("Example of add");
    println!("{}", v + w);
    // vector scalar multiply
    println!("Example of scalar multiply");
    println!("{}", v * 3.0);
    println!("{}", 2.0 * v);

    // Example of matrix
    println!("Example of matrix");
    // matrix definition
    let i = Mat3::from_cols_array_2d(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let j = Mat3::from_cols_array(&[2.0, 3.0, 1.0, 4.0, 6.0, 5.0, 9.0, 7.0, 8.0]);
    // matrix output
    println!("Example of output");
    println!("{}", i);
    println!("{}", j);
    // matrix add i + j
    // matrix scalar multiply i * 2.0
    // matrix multiply i * j
    // matrix multiply vector i * v
}
