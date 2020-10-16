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

The `let y = 6` statement doesn't return a value, so there isn't anything for `x` to bind to.  This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.  In those languages, `x = y = 6` is valid and would assign the value `6` to both `x` and `y`; not so in Rust.  

Expressions evaluate to something and make up most of the rest of the code that you'll write in Rust.  Consider a simple math problem, such as `5 = 6`, which is an expression that evaluates to the value `11`.  Expressions can be part of statements: in `let y = 6;` this is an expression that evaluates to `6`.  Calling a function, or macro is an expression.  Creating new scopes with `{}` is an expression.  EX:  

    fn main() {
        let x = 5;

        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);
    }

This expression:  

    {
        let x = 3;
        x + 1
    }

is a block that evaluates to `4`.  That value gets bound to `y` as a part of the `let` statement.  Note the `x + 1` line without a semicolon at the end, which isn't like most lines seen so far.  Expressions do not include ending semicolons.  If you add a semicolon to the end of an expression, it becomes a statement, which won't return a value.  

####### Functions with Return Values #######  

Functions can return values to the code that calls them.  We don't name returned values, but we do declare their type after an arrow (`->`).  In Rust, the return value of the functions is synonymous with the value of the final expression in the block of the function body.  You can return early from a function by using the `return` keyword and sepcifying a value, but most functions just return the last expression implicitly.  EX of a function that returns a value:  

    fn five() -> i32 {
        5
    }

    fn main() {
        let x = five();

        println!("The value of x is: {}", x);
    }

There are no function calls, macros, or even a `let` statement in the `five` function -- just the number `5` by itself.  That's a perfectly valid function in Rust.  Note that the return type is specified as `-> i32`.  The `5` in `five` is the function's return value, which is why the return type is i32.  Let's examine in more detail.  There are two important bits: first, the line `let x = five();` shows that we're using the return value of a function to initialize a variable.  Because the function `five` returns a `5`, the line is the same as `let x = 5;`  

Second, the `five` function has no params and defines the type of the return value, but the body of the function is just `5` because it's an expression whose value we want to return.  

Another example:  

    fn main() {
        let x = plus_one(5);

        println!("The value of x is: {}", x);
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }  

Running this will print `The value of x is: 6`.  But if we place a `;` at the end of the line `x + 1`, we'll get an error like so: 

    cargo run
       Compiling functions v0.1.0 (file:///projects/functions)
    error[E0308]: mismatched types
     --> src/main.rs:7:24
      |
    7 | fn plus_one(x: i32) -> i32 {
      |    --------            ^^^ expected `i32`, found `()`
      |    |
      |    implicitly returns `()` as its body has no tail or `return` expression
    8 |     x + 1;
      |          - help: consider removing this semicolon

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `functions`.

    To learn more, run the command again with --verbose.  

The main message, "mismatched types," reveals the core issue.  The definition of the function `plus_one` says that it will return an `i32`, but staements don't evaluate to a value, which is expressed by `()`, an empty tuple.  Therefore, nothing is returned, which contradicts the function definition and results in an error.  Rust suggest removing the semicolon, which would fix the error.