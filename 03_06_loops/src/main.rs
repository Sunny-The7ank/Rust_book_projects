fn main() {

    let mut loop_counter = 0;

    loop {
        println!("again!");
        loop_counter += 1;
        if loop_counter >= 3 {
            break;
        }
    }
    
}