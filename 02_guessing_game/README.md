####### Processing a Guess #######  

The first part of the game program will ask for user input, process that input, and check that the input is in the expected form.  To start, we'll allow the player to input a guess.  

`Listing 2-1 code`

Some new concepts!  Let's go over line by line.  

To obtain user input and print the result as output, we need to bring the `io` (input/output) library into scope.  The `io` library comes from the standard library `std` 

    use std::io;

By default, Rust only brings a few types into the scope of every program in the prelude (google rust prelude).  If something you need isn't in the prelude, bring it into scope using the `use` statement.  The `std::io` library provides number of features including the ability to accept user input.
Main function `fn main() {` and `println!` are old news, skipping.

####### Storing Values with Variables #######  

Next, we'll create a variable to store user input:  

    let mut guess = String::new();

A `let` statement is used to create a new variable.  By default, variables are immutable.  This means that they are not changeable.  We can set a variable as mutable by starting variable declaration with `let mut`

`String::new()` returns a new instance of a `String` type.  Strings are a growable UTF-8 encoded bit of text.

The `::` syntaxs denotes that `new` is an associated function of the String type. (AKA static method)

The `new` function creates a new, empty string.  

Now for the i/o part.