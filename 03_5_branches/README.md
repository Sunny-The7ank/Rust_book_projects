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

