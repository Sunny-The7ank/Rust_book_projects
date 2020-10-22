####### Repetition with Loops #######  

It's often usefule to execute a block of code more than once.  For this, Rust provides three types of loops: `loop`, `while`, and `for`.  

####### Repeating Code with `loop` #######  

The `loop` keyword tells rust to execute a block of code over and over forever or until explicitly told to stop.  

    fn main() {
        loop {
            println!("again!");
        }
    }

When we run this, we see `again!` repeated on a new line until we stop the program manually using ctrl+c.  

Rust provides a cleaner, more reliable method fro breaking out of a loop called `break`.  This should be used after a certain condition is met to end the loop.  

####### Returning Values from Loops #######  

One of the uses of a `loop` is to retry an operation that might fail, such as checking for thread completion.  However, you might need to pass the result of that operation to the rest of your code.  To do this, you can add the value you want to return after the `break` expression used to stop the loop; that value will be returned out of the loop for use, EX:  

    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }  

Before the loop, we declare `let mut counter = 0;`  Then we declare `result` to hold the value returned from the loop.  On every iteration of the loop, we add `1` to `counter`, and check if it is equal to `10`.  When it is, we break and return `counter` multiplied by `2`, which is 20.  

####### Conditional Loops with `while` #######  

It's often useful for a program to evaluate a condition within a loop.  While the condition is true, the loop runs like normal.  When the condition is no longer true, call `break`, and stop the loop.  This loop type could be implemented using a combination of `loop`, `if`, `else`, and `break`.  But, Rust already has something for this called a `while` loop.  EX:  

    fn main() {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

This construct eliminates a lot of the nesting that would be necessary if you used other building blocks to kludge it.  

####### Looping Through a Collection with `for` #######  

You could use a `while` loop to iterate over elements of a collection, such as an array.  EX:  

    fn main() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }
    }

Here, the code counts up through the elements in the array.  It starts at index `0`, and loops until it reaches the final index in the array (when `index < 5` is no longer true).  

All five array values appear in the terminal, as expected.  Even though `index` will reach a value of `5` at some point, the loop stops executing before trying to fetch a sixth value from the array (which would crash).  An index of 4 would refer to the final value, because arrays are 0 indexed.  

This approach is error prone; we could cause the program to panic if the index length is incorrect.  It's slow, because the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.  

A more concise alternative would be a `for` loop and to execute some code for each item in the collection.  EX:  

    fn main() {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

When run, this code produces the same output as the `while` loop, but now the loop is much safer.  If the size of a collection accessed with a `while` loop is changed, the conditional needs to be changed.  With a `for` loop, we are iterating over each item in the collection in turn.  The `for` loop is also more concise.  

The safety and conciseness of `for` loops make them the most commonly used type of loop in Rust.  Even in situations where you want to run a piece of code a certain number of times, as in the countdown example from earlier, mots Rust devs would use a `for` loop.  The way to implement this is with a `Range`, which is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number.  

Here's what that countdown would look like, using `for` and another method we haven't seen `rev`, which reverses a range.  

    fn main() {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
