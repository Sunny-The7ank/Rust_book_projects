fn main() {

    // First loop example
    let mut loop_counter = 0;

    loop {
        println!("again!");
        loop_counter += 1;
        if loop_counter >= 3 {
            break;
        }
    }

    // Returning values from loops example
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While loop example
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // While loop vs for loop example

    //while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //for loop
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}