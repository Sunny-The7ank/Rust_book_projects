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

####### Shadowing #######  

You can declare a new variable with the same name as a previous one, and the new variable shadows the previous one.  *Shadowing* means that the first declaration is shadowed by the second, which means the the second variable's value is what appears when the variable is used.  We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword like so:  

    fn main() {
        let x = 5;

        let x = x + 1;

        let x = x * 2;

        println!("The value of x is: {}", x);
    }  

This program first binds `x` to the value `5`.  It then shadows `x` by repeating `let x =`, taking the original value and adding `1` so `x` = `6`.  The third statement shadows a shadowed value and multiplies `x` by `2` to give `x` a final value of `12`.

Shadowing is different from making variables mutable, because we'll get a compiler error if we try to reassign a variable without using the `let` keyword.  By using `let`, we can perform a few transformations on a value but have the variable immutable after those transformations have been completed.  

The other difference between `mut` and shadowing is that because we're effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.  EX: say our program asks a user to show how many spaces they want between some text by inputting space characters, but we really want to store that input as a numerical value:  

    let spaces = "   ";
    let spaces = spaces.len();  

This construct is allowed because the first `spaces` variable is a string type and the second `spaces` variable, which is a brand-new variable that happens to have the same name as the first one, is a number type.  Shadowing spares us from having to make a new variable and new names such as `spaces_str` for the input, and `spaces_num` for the number of spaces; instead, we can reuse the simpler `spaces` name.  However, if we try to use `mut` for this, we'll get a compiler error.  

    let mut spaces = "   ";
    spaces = spaces.len();  

    $ cargo run
    Compiling variables v0.1.0 (file:///projects/variables)
    error[E0308]: mismatched types
    --> src/main.rs:3:14
      |
    3 |     spaces = spaces.len();
      |              ^^^^^^^^^^^^ expected `&str`, found `usize`  

This error means we can't mutate a variable's type.