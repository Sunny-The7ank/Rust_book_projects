####### Control Flow #######  

The basic building blocks of a programming language are made up of deciding whether or not to run some piece of insturctions based on some conditions or continually running a piece of code until a condition is met.  The most common constructs that let you control execution flow are `if` expressions and loops.  

####### `if` Expressions #######  

An `if` expression allows you to branch your code depending on some condition.  You provide a condition and then state "if this condition is met, run this.  If not, run this."  

Put this in your main.rs:  

    fn main() {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }  

All `if` expressions start with the `if` keyword, which is followed by a conditional.  In this example, the condition checks if the variable `number` is less than `5`.  The block of code run if the statement is true is placed immediately after the condition check inside curly braces `{}`.  Blocks of code associated with the conditions in `if` expressions are sometimes called arms, just like arms in a `match` statement.  

Optionally we can include an `else` expression, which is executed if the condition is not met.  If an `else` expression is not provided, the program will just skip the `if` block and move on to the next bit of code.  

It's worth noting that the condition in this code must be a `bool`.  If the condition isn't a bool, we'll get an error like this:  

    fn main() {
        let number = 3;

        if number {
            println!("number was three");
        }
    }  

The `if` condition evaluates to `3`, error is:  

    Compiling branches v0.1.0 (file:///projects/branches)
    error[E0308]: mismatched types
     --> src/main.rs:4:8
      |
    4 |     if number {
      |        ^^^^^^ expected `bool`, found integer

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `branches`.

    To learn more, run the command again with --verbose.  

The error indicates that Rust expected a `bool` but got an integer.  Unlike langs like Ruby and JavaScript, Rust doesn't automatically convert non-boolean values to booleans.  You must be explicit and always provide `if` with a boolean as its condition.  If we want the code block to run only when a number is not equal to zero, we can change the if expression to the following:  

    fn main() {
        let number = 3;

        if number != 0 {
            println!("number was something other than zero");
        }
    }  

####### Handling Multiple Conditions with `else if` #######  

You can have multiple conditions by combining `if` and `else` in an `else if` expression.  For example:  

    fn main() {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }  

The program has four possible paths it can take.  This is the output from running with default value:  

    Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
    number is divisible by 3  

When this program executes, it checks each `if` expression in turn and executes the first body where the condition is true.  We don't see the other matching arms since `if` statements only execute the FIRST match.  

Using too many `else if` expressions can clutter code, so if you have more than one, consider refactoring to a `match` expression.  

####### Usinf `if` in a `let` Statement #######  

Because `if` is an expression, we can use it on the right side of a `let` statement like this:  

    fn main() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {}", number);
    }