####### Hello, Cargo! #######
Cargo is Rust's build system and package manager. Most rust devs use it to manage projects 
because it handles a lot of tasks for you, such as building code, downloading libraries, and
building those libraries.
(Libraries that your code needs are called dependencies)

The simplest programs like "Hello, world!" don't have any dependencies. More complex programs
will require the use of cargo. 

####### Creating Projects with Cargo #######
Basic project creation in cargo uses this syntax:
    cargo new <project_name>
This creates a new project named whatever project_name is in a new directory named 
project_name. 
Cargo generates 2 files and one directory for us: a Cargo.toml file, and a src directory
with a main.rs file inside. 
It also initializes a new Git repo along with a .gitignore. Git files won't be generated
if you run cargo new inside an existing repo; you can override with: cargo new --vcs=git