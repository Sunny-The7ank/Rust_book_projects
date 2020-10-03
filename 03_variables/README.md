####### Variables and Mutability #######  

By default, variables are immutable.  This is a nudge by Rust toget you to write code that takes advantage of safety and easy concurrency that Rust offers.  You still have the option to make your variables mutable and there are many reasons to do so if neccessary.  
When a variable is immutable, it's value is set and can't be changed.  
Let's add some code to explore immutability:  
    
    fn main() {
        let x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

This code won't compile because you cannot assign a value to an immutable variable twice.  The compiler won't actually run this code.  This example shows how the compiler helps you find errors.  
Variables are immutable only by default in Rust.  We can get this code to compile and run if we change our first variable assignment to include `mut` after let.  In addition to allowing you to change the value of a variable during runtime, `mut` conveys to future readers that other parts of the code may change the value of the variable.  

There are tradeoffs for using mutability.  With large data structures, it may be faster to mutate an instance in place rather than copying the whole thing.  In small data structures it may be easier to create new instances.  

####### Differences Between Variables and Constants #######  

Constants are variables that you set before compilation.  These values never change.  They are always immutable.  An example of setting a constant:  

    const MAX_POINTS: u32 = 100_000;  

The naming convention for constants is to be all upper case, use snake_case for the variable name, and use underscores in numerical values to help with readability.  

