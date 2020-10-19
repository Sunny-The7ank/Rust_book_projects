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