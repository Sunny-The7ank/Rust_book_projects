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

In the Cargo file, a section extends until another section is declared.  The `[dependencies]` section is where you tell cargo what external crates your program requires.  We are specifying the `rand` crate and the version `0.5.5` specifically.  The number `0.5.5` is shorthand for `^0.5.5`, which means "any version that has a public API compatible with version 0.5.5"  

Now we will build the program and see Cargo import `rand` and anything it depends on.  It should look similar to this:  

    Compiling winapi v0.3.9
    Compiling rand_core v0.4.2
    Compiling rand_core v0.3.1
    Compiling rand v0.5.6
    Compiling guessing_game v0.1.0 (D:\Users\<username>\rust_book_projects\02_guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 10.88s

Cargo automatically fetches external crates and dependencies from Crates.io.  Crates.io is where most developers post their open source Rust projects and libraries for others to use.  Cargo only fetches new crates/dependencies if there is a change.  

####### Ensuring Reproducible Builds #######  

Carco has a mechanism where you can lock dependencies at a certain level.  This is desireable if a new version of a crate breaks something in your code.  This also ensures that anyone else who builds your code will have the same output.  This is done automatically by Cargo.  When you build after including some new dependencies, the dependencies are added to the `Cargo.lock` file.  The crates you import will stay at the level you choose until you explicitly upgrade.  

When you want to upgrade a crate, that can be done by running `cargo update`.  This ignores the *Cargo.lock* and figures out the latest versions that fit your specs within the *Cargo.toml* file.  By default, in this instance with rand, Cargo will only look for versions higher than `0.5.5` and lest than `0.6.0`.  If you want to use a `rand` version `0.6.0` or any version in the `0.6.x` series, you'd have to update the *Cargo.toml* file to look like this:  

    [dependencies]
    rand = "0.6.0"

The next run of `cargo build` will make Cargo update the internal crate registry.

####### Generating a Random Number #######  

Now that `rand` is added, let's use it.  See listing 2-3 in the book for changes.  

The first thing we did was add `use rand::Rng`.  The `Rng` trait defines methods that random number gens implement.  This trait must be in scope to be able to use it's methods.  

Next, we added 2 lines.  The `rand::thread_rng` function will give us the particular random number generator that we're going to use: one that is local to the current thread of execution and seeded by the OS.  Then we call `gen_range` to set an upper and lower bound for our random number: 1 - 100.  But, we must use 1  - 101 to get a number between 1 and 100.  

The second line is a debug line that lets us know what the current secret number is.  This will be deleted later to make this an actual guessing game.

####### Comparing Input to the Secret Number #######  

Adding the code from Listing 2-4.  This code doesn't compile... yet.  

The first thing we're doing is bringing in another library from the standard lib: `std::cmp::Ordering`.  This is another enum like Result.  It's variants are `Less`, `Greater`, and `Equal`.  

Next, we add 5 lines for a comparison between `guess` and `secret_number`.  We are using a `match` expression for matching.  Match expressions are made of arms.  An arm contains a pattern and the code that should run in the event of a match.  

However, this code won't even compile.  This is because of a type mismatch between the `guess` variable and the `secret_number` variable.  When we wrote `let mut guess = String::new()`, it was inferred to be a String.  The `secret_numer`, however, is a number type.  Ultimately, we want to conver the String from stdin into a number type so we can compare numerically in the match statement.  We'll add this line to make it convert:  

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

We created another variable named `guess`.  There is already a variable named that but, Rust allows us to *shadow* the previous value of `guess` with a new one.  `Shadowing` is often used when you want to convert variable types without the need to create a new variable.  Let's walk through this.  
`let guess: u32` is shadowing the previous `guess` variable as a `u32` type.  We bind `guess` to the expression `guess.trim().parse()`.  This `guess` refers to the original that was a `String` with the user input in it.  The `trim()` method on a `String` will eliminate any leading and trailing whitespace.  Although `u32` can only contain numerical chars, the user must press enter to satisfy `read_line`.  When enter is pressed, a newline char is added to the input.  Ex: if you type in 5 and press enter, `guess` contains: `5\n`.  Trim also eliminates the newline char.  

The `parse` method on strings parses a string into a number type.  BEcause this method can be used for multiple different numerical types, we need to tell Rust what type of number we want.  For this example, a `u32` is sufficiently sized because it is an unsigned 32-bit integer.  

Because `parse` calls could easily result in errors, it also returns a `Result` type.  We will handle this type much like the `read_line` result previously.  Any `Err` returned will crash the game.  Now let's make it run until you quess correctly or rage quit.  

####### Allowing Multiple Guesses with a Loop #######  

The `loop` keyword creates an infinite loop.  Let's modify the code for that.  Ex:  

    // --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
    // --snip--  

Now that everything is in a loop, make sure to indent lines correctly.  Unfortunately, the only way to quit right now is to crash the game.  Let's make the exit more graceful by modifying the match statement.  Here's what that should look like:  

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }