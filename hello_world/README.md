####### Anatomy of a Rust program #######  

    fn main() {  
    
    }  
This is a function in rust.  Specifically, the main function. Every program in rust
needs a main function and can only have one. The first line main() declares main
as a new function that has no params and doesn't return anything. Params would go
in the parens. 

The func body is wrapped in curly braces {}. Rust requires these around all func bodies.
It's good style to place the open brace on the same line as func declaration, adding one
space between  
  
    println!("Hello, world!");  
This line does all the work in this little program: it prints text to the screen.  
There are 4 important details to note:  
1. Rust style is to indent with 4 spaces, not tabs.  
2. println! calls a macro. If it called a func, it wouldn't have a !  
3. We pass strings as an argument to println!, and the string is printed to the screen
4. We end lines with ; which indicates that this expression is over and the next can begin.  Most, but not all, lines end with a semicolon  

####### Compiling and Running #######  
These are 2 different steps. Cargo can make it look like 1 if you use cargo build. 
On windows, when compiled, rustc generates 2 additional files when compiled: a .exe and a .pdb
The .exe is the executable program. The .pdb is debug info.
