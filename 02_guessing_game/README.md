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

    io::stdin()
        .read_line(&mut guess)

If you don't put the `use std::io` at the top, you'd have to write the function call as `std::io::stdin`.  The `read_line` part calls the `read_line` method on the standard input handle to get input from the user.  Also passing one arg to it, `&mut guess`  

The job of this line is to take whatever is input from standard input (the console) and place it into a string.  The String arg needs to be mutable so that the method can change the content of the string by adding the user's guess.  

The `&` tells us that this argument is a reference, which lets multiple parts of the code to access one pieve of data without copying it (saves on memory space).  Reference are immutable by default so that's why we use `&mut guess` rather than `&guess`.  

####### Handling Potential Failure #######  

The next part of the method:  

    .expect("Failed to read line");

When you call a method with the `.foo()` syntax, it's best to insert a line rather than having your line look like this:  

    io::stdin().read_line(&mut guess).expect("Failed to read line");

`read_line` puts user input into a string that we are passing, but it also returns a value in the form of an `io::Result`.  Rust has a number of types named Result in its standard library: a generic Result as well as specific submodules, such as `io::Result`.  

The Result types are enumerations, often referred to as enums.  An enum is a type with a fixed set of values.  These values are known as the enum's variants.  

For `Result`, the variants are `Ok` or `Err`.  The OK variant means the operation was successful, and inside Ok is the successfully returned value.  The Err variant means a failure, and the Err contains info about the failure.  

The purpose of these result types is to encode error-handling info.  Values of the `Result` type, like values of any type, have methods defined on them.  An instance of io::result has an `expect` method that can be called.  If the instance of `io::Result` is an `Err`, `expect` will cause the program to crash and display the message you pass as an arg.  If you don't build in a handler for an `Err` Result, your program will compile, but you will get an error.  The right way to suppress warnings is to actually write error handling, but if you just want the program to crash out, you can just use `expect`.  

####### Generating a Secret Number #######  

Right now, the program just accepts an input from standard in and echos it back to standard out.  To actually make this do something, we need to import a `crate`.  Specifically the `rand` crate.  This crate allows us to generate a random number for comparison to the input.  

A crate is just a collection of Rust source code files.  The project we've been building is a *binary* crate, meaning it is meant to be an executable.  The `rand` crate is a *library* crate, which is a collection of code intended to be used in other programs.  

Before we can write code to take advantage of the rand crate, we need to import it as a dependency.  Open the `Cargo.toml` file and add the following to the bottom.  

    [dependencies]
    rand = "0.5.5"