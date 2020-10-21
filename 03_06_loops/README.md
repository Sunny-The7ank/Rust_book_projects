####### Repetition with Loops #######  

It's often usefule to execute a block of code more than once.  For this, Rust provides three types of loops: `loop`, `while`, and `for`.  

####### Repeating Code with `loop` #######  

The `loop` keyword tells rust to execute a block of code over and over forever or until explicitly told to stop.  

    fn main() {
        loop {
            println!("again!");
        }
    }

When we run this, we see `again!` repeated on a new line until we stop the program manually using ctrl+c.  

Rust provides a cleaner, more reliable method fro breaking out of a loop called `break`.  This should be used after a certain condition is met to end the loop.  

