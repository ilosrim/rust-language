use std::any::type_name;
// use std::fmt::Display;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#![allow(unused)]
fn main() {
    // Data Types
    // Scalar Types:
    // integers, floating-point numbers, Booleans, characters
    
    // FLOAT
    let x: i64 = 21; // default 32
    let y: f32 = 2.5; // default 64
    println!("{}", type_of(x));
    println!("{}", type_of(y));

    // INTEGERS
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;
 
    // multiplication
    let _product = 4 * 30;
 
    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0
 
    // remainder
    let _remainder = 43 % 5;

    // BOOLEAN
    let _t = true;
    let f: bool = false;
    println!("{}", f);

    // The Character Type
    let c = 'z';
    let _z = 'Z';
    println!("{}", type_of(c));

    // COMPOUND TYPES
    // Tuple
    let tup: (i32, f64, u8) = (500, 2.5, 1);
    println!("{:?}", tup);
    println!("{:?}", type_of(tup));

    let tup2 = (600, 3.7, 2);
    let (_l,m,_n) = tup2;
    println!("The value of y is: {}", m);

    let k: (i32, f64, u8) = (500, 5.2, 1);
    let _five_hundred = k.0;
    let _five_point_two = k.1;
    let _one = k.2;

    // Arrays
    let arr: [i32, 5] = [1,2,3,4,5];
}
