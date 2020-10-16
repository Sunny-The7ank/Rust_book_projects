fn main() {
    println!("Hello, world!");

    another_function();
    another_other_function(5);
    another_function_again(5, 6);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

// Function parameters
fn another_other_function(x: i32) {
    println!("The value of x is: {}", x);
}

// 2 function params
fn another_function_again(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}