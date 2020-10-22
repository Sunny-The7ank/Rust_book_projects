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
}