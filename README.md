# ++Time Rust Tutorial

### Outline:

#### Introduction:
- Installation [DONE]
- Cargo [DONE]
- Hello World
- Functions
- Moving, Borrowing, Lifetimes

#### What makes Rust Rust
- Options / Results (Parse command line input)
- Match

#### Example
- Pull together concepts
- Introduce thread constructs

### Installation
#### Linux/Mac
For Linux and Mac, there is a convenient script:

    curl https://sh.rustup.rs -sSf | sh

#### Windows
Obtain and run the [Rust Installer](https://win.rustup.rs/). 

For all platforms, it will update the necessary scripts to set up your `PATH`, but as it mentions for Linux and Mac there will be an additional step to get your current session working. Try using `cargo --version` to verify the installation. Visit the [installation page](https://www.rust-lang.org/en-US/install.html) on the Rust website if you have any difficulty. 

You'll probably want syntax highlighting (at minimum). Check [here](https://areweideyet.com/) for plugins and instructions for your favorite editor.

### The obligatory "Hello, World!"
Open up a terminal and navigate to where you'd like to start your first Rust project. Run `cargo new hello --bin`. I've now mentioned `cargo` twice. `cargo` is Rust's main tool suite. You'll use `cargo` for everything from compiling to package management. The command you just ran created a new project with its own folder structure:

    +hello
    ├—+src
    │ └—main.rs
    └—Cargo.toml

The top-level folder is the name of the package. Most names in Rust should be in *snake_case*, and there will be compiler warnings abound to remind you unless you supress them. The *Cargo.toml* file defines all of your dependencies, and is a bit like Node's *package.json*. Libraries in Rust are called **crates**. There is a healthy repository of crates at [crates.io](https://crates.io/). The *src* directory is the root of your source code. *main.rs* has your program entry point, and was created because of the `--bin` option from the `cargo new` command. Had you omitted the `--bin`, `cargo` would have created a **library**, and there would be a *lib.rs* in  place of *main.rs*.

Let's run your first Rust program. `cd` into the *hello* directory and run `cargo run`. This will compile and run the *hello* package, and you should see something like this:

    bash-3.2$ cargo run
    Compiling hello v0.1.0 (file:///Users/alexbowen/Code/Personal/RustTutorial/hello)
     Finished debug [unoptimized + debuginfo] target(s) in 7.42 secs
      Running `target/debug/hello`
    Hello, world!
    bash-3.2$
You'll see that there is a new directory called *target*, containing another folder called *debug*, which contains an executable called *hello* (or *hello.exe* for the Windows folk).

We're finally ready to take a look at some code. If you are using a folder-based text editor like Sublime or VSCode, you'll want to open up the *hello* folder. Editing *main.rs*, you'll see:
```rust
fn main() {
    println!("Hello, world!");
}
```