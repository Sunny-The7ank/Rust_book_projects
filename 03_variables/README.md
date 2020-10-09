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

####### Data Types #######  

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.  There are 2 data type subsets: scalar and compound.  

Rust is a statically typed language meaning it must know the types of all variables at compile time.  The compiler can usually infer what type we want based on the value and how it's used.  In cases where many types are possible, such as when converting a `String` to a numeric type using `parse`, we must use a type annotation like this:  

    let guess: u32 = "42".parse().expect("Not a number!");  

If we don't add an annotation, the compiler will give an error saying `type annotations needed`.  

####### Scalar Types #######  

A scalar type represents a single value.  There are four primary scalar types in Rust: integers, floating-point numbers, booleans, and characters.  

####### Integer types #######  

An integer is a number without a fractional component... Basically any whole number.  Integers can be signed or unsigned.  Unsigned can only hold positive numbers.  Signed can be positive or negative.  Integers can be 8, 16, 32, 64, or 128 bits in length.  There is also the `isize` and `usize` which has a length that is defined by the architecture that the program is running one: 32-bit or 64-bit.  Signed variants can store numbers from -(2^(n-1)) to 2^(n-1) -1 inclusice, where n is the number of bits.  You can write integers in decimal, hex, octal, binary, and byte (only for u8).  Rust defaults to i32, which is generally the fastest, even on 64-bit systems.  The primary use for `isize` or `usize` is when indexing some kind of collection.  

Note: if you have a program where a variable is a `u8` (holds a value 0-255 inclusive) and you try to change the value to 256, an integer overflow occurs.  Rust has some interesting rules with this.  When compiled in debug mode, Rust has a check for integer overflows and will fail at compilation.  If you compile with the `--release` flag enabled, Rust does not include those checks and will compile successfully.  If an overflow occurs, Rust performs two's complement wrapping.  Values greater than the maximum value of the type wrap around to the minimum value of the type.  In the case of a `u8`, 256 becomes 0, 257 becomes 1, and so on.  Relying on integer wrapping is a very bad idea.  

####### Floating-point Types #######  

Rust has two primitive types for floating point numbers, which are numbers with decimal points.  Rust's floating points are either `f32` or `f64`, which are 32-bits and 64-bits in size, respectively.  The default type is `f64` because on modern CPUs it's roughly the same speed as `f32` but has more precision.  The `f32` type is a single-precision, and `f64` has double precision.  

####### Numeric Operations #######  

Rust supports all the basic math operations you'd expect: addition, subtraction, multiplication, division, and remainder (modulo).  

####### Boolean type #######  

Booleans are either `true` or `false`, moain way to use these is through conditionals, such as `if` expressions.  

####### The Character Type #######  

Rust's `char` type is the most primitive alphabetic type.  `char` literals are specified with single quotes, as opposed to string literals, which use double quotes.  The `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.  Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust.  Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.  However, a "character" isn't really a concept in Unicode, so your intuition for what a "character" is may not match up with what a `char` is in Rust.  

####### Compound Types #######  

Coumpound types can group multiple values into one type.  Rust has two primitive compound types: tuples and arrays.  

####### The Tuple Type #######  

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.  Tuples have a fixed length: once declared, they cannot change size.  

We create a tuple by writing a comma-separated list of values inside parentheses.  Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same.  EX:  

    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }  

The variable `tup` binds to the entire tuple, because a tuple is considered a single compound element.  To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:  

    fn main() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {}", y);
    }  

This program first creates a tuple and binds it to the variable `tup`.  It then uses a pattern with `let` to take `tup` and turn it into three separate variables, `x`, `y`, and `z`.  This is called destructuring, because it breaks the single tuple into three parts.  Finally, the program prints the value of `y`, which is `6.4`.  

In addition to destructuring through pattern matching, we can access a tuple element directly by using period(`.`) followed by the index of the value we eant to access.  EX:  

    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }  

This program creates a tuple `x`, and then makes new variables for each element by using their respective indices.  Tuples are zero indexed.  

####### The Array Type #######  

Another way to collect multiple values is with arrays.  Unline tuples, arrays are all of the same type.  Arrays in Rust are fixed in length.  Values going into an array are written as comma separated within square braces:  

    let a = [1, 2, 3, 4, 5];  

Arrays are useful when you want your data on the stack rather than the heap.  An array isn't as flexible as the vector type, which is able to change in size.  If you don't know whether to use a vector or an array, you should probably use a vector.  
An example of when you would definitely use an array over a vector is in a program that needs to know the months of the year.  

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];  

You would write an array's type by using square brackets, and within them include the types for the array members and how many members like this:  

    let a: [i32; 5] = [1, 2, 3, 4, 5];  

We defined that the array `a` has 5 elements, all the type `i32`.

Writing an array's type like this looks similar to an alternative syntax for initializing an array: if you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array.  EX:  

    let a = [3; 5];
    // is the same as: 
    let a = [3, 3, 3, 3, 3];  

The array named `a` will contain 5 elements all set to 3.  This is much more concise.  

An array is a single chunk of memory on the stack.  You can access elements using indexing.  

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];  

In this example, the variable `first` will get the value `1`, and `second` will get the value `2`.

What happens if you try to access an element of an array that is past the end of the array?  The program will compile, but it will panic at runtime.  

    fn main() {
        let a = [1, 2, 3, 4, 5];
        let index = 10;

        let element = a[index];

        println!("The value of element is: {}", element);
    }  

Running `cargo run` will produce this:  

    thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src\main.rs:88:19
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\variables.exe` (exit code: 101)  

The compiler didn't catch the error, but the program resulted in a runtime error and didn't exit successfully.  When you attempt to access an array out of bounds, Rust will check that your index specified is within the bound of the array.