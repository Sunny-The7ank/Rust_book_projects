#![allow(unused_variables, dead_code)]
fn main() {

    // Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;

    // Shadowing
    let shadow = 5;

    let shadow = shadow + 1;

    let shadow = shadow * 2;

    println!("The value of x is: {}", shadow);

    let spaces = "   ";
    let spaces = spaces.len();

    // Floating-point Types
    let x_float = 2.0; // f64

    let y_float: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of y is: {}", b);

    let x_tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x_tup.0;

    let six_point_four = x_tup.1;

    let one = x_tup.2;
}