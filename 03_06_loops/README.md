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

####### Returning Values from Loops #######  

One of the uses of a `loop` is to retry an operation that might fail, such as checking for thread completion.  However, you might need to pass the result of that operation to the rest of your code.  To do this, you can add the value you want to return after the `break` expression used to stop the loop; that value will be returned out of the loop for use, EX:  

    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }  

Before the loop, we declare `let mut counter = 0;`  Then we declare `result` to hold the value returned from the loop.  On every iteration of the loop, we add `1` to `counter`, and check if it is equal to `10`.  When it is, we break and return `counter` multiplied by `2`, which is 20.  

####### Conditional Loops with `while` #######  

It's often useful for a program to evaluate a condition within a loop.  While the condition is true, the loop runs like normal.  When the condition is no longer true, call `break`, and stop the loop.  This loop type could be implemented using a combination of `loop`, `if`, `else`, and `break`.  But, Rust already has something for this called a `while` loop.  EX:  

    fn main() {
        let mut number = 3;
    
        while number != 0 {
            println!("{}!", number);
    
            number -= 1;
        }
    
        println!("LIFTOFF!!!");
    }