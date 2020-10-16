####### Functions #######  

Functions are pervasive in Rust code.  One of the most important functions is the `main` function.  Also have seen that functions begin with the `fn` keyword.  

Rust uses snake case for function and variable names.  In snake case, all words are lower case, and words are separated with `_`.  

    fn main() {
        println!("Hello, world!");

        another_function();
    }

    fn another_function() {
        println!("Another function.");
    }

All function definitions start with `fn` and have a set of parens after the function name.  The curly braces tell the compiler where the function body begins and ends.  

We can call any function we've defined by entering it's name followed by parens.  Because `another_function` is defined within our program, it can be called from inside the main function.  Note that functions can be defined before or after the main function in source code.  Rust doesn't care where in the code functions are defined, only that they are.

####### Function Parameters #######  

Functions can also be defined to have parameters, which are special variables that are part of a function's signature.  When a function has parameters, you can provide it with concrete values for those params.  Technically, these are called function arguments, but argument and param are interchangeable for a function definition.  EX:  

    fn main() {
        another_function(5);
    }

    fn another_function(x: i32) {
        println!("The value of x is: {}", x);
    }

The declaration of `another_function` has one param named `x` of type `i32`.  When `5` is passed to `another_function`, the `println!` macro puts `5` where the curly braces are in the format string.

In function sigs, you must declare the type of each param.  This is a deliberate decision in rust's design: requiring type annotations means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.  

When you want a function to accept multiple params, just separate them with commas.  EX:  

    fn main() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }  

This example creates a function with two params, both are `i32`.  The function then prints the values in both of its params.  Note: the function params don't need to be all the same type.  

####### Function Bodies Contain Statements and Expressions #######  

So far, we've only done functions in Rust without an ending expression, but we've used expressions as part of a statement.  Rust is an expression based language, so it's important to know the difference between expressions and statements.  Statements are instructions that perform an action and don't return a value.  Expressions evaluate to a value.  Example time:  

Creating a variable and assigning a value with the `let`  keyword is a statement.  
ex: `let y = 6;`

Function definitions are also statements; this entire thing is a statement in itself:  

    fn main() {
        let y = 6;
    }  

Statements don't return values.  Therefore, you can't assign a `let` statement to another variable, as the following code tries to do; you'll get an error:  

    fn main() {
        let x = (let y = 6);
    }

    $ cargo run
       Compiling functions v0.1.0 (file:///projects/functions)
    error: expected expression, found statement (`let`)
     --> src/main.rs:2:14
      |
    2 |     let x = (let y = 6);
      |              ^^^
      |
      = note: variable declaration using `let` is a statement
