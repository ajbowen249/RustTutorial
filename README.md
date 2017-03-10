# ++Time Rust Tutorial

### Outline:

#### Introduction:
- Installation [DONE]
- Cargo [DONE]
- Hello World [DONE]
- Mutability [DONE]
- Functions [DONE]
- Moving, Borrowing, Lifetimes
- Traits

#### What makes Rust Rust
- Options / Results (Parse command line input) [DONE]
- Match [DONE]

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
This should all be pretty self-explanatory. The most iteresting thing is the exclaimation point and the end of `println!`. When you see an `!` in a Rust function call, you're actually calling a macro. Rust has a very robust macro system that supplements syntactic ease that many languages achieve by relaxing type safety. 

### Let's do a Rust
Rust's basic syntax is primarily c-like. Add this to your *main.rs*:
```rust
    let x = 12;
    let y = 15;
    let z = x + y;

    println!("{} + {} = {}", x, y, z);
```
Again, this is all pretty self-explanatory, and the interesting part is in the `println`. Rust macros that facilitate string formatting usually take a variadic set of arguments following the pattern string that are inserted in place of the `{}` in the format string in order. The macro system and the compiler are clever enough to give you a compiler error if you supply the wrong number of arguments or a type that has no default string representation. 

X, y, and z are all integers, specifically `i32`s. When declaring variables of primitive types, Rust has a few default types. You could have declared x with:
```rust
    let x: i32 = 12;
```

Rust has three core numeric types: [u]nsigned, [i]nteger, and [f]loats. When you want a specific type, it's usually a good guess that the type name is a letter and a size, such as `u8`, `i64`, `f32`, `u64`, `f16`, and so on. You'll rarely need type hints when declaring and initializing on the same line, thanks to Rust's strong degree of type safety, even for non-primative types. Add this code:
```rust
    let mut a = 5;
    a = a + 1;
    println!("a: {}", a);
```

The big deal here is the `mut` keyword. The project will not compile if you remove it. Variable bindings in Rust must explicitly specify their mutability. It is one of the most important concepts behind what makes Rust Rust. This tracking allows the compiler to guarantee that there are **zero** data races in non-unsafe code. 

Let's touch briefly on vectors:
```rust
    let nums = vec![4, 8, 15, 16, 23, 42];

    for i in 0..nums.len() {
        println!("{}: {}", i, nums[i]);
    }
```

Vectors are the standard collection type in Rust. The `vec!` macro can be used to initialize a vector. This creates a `Vec<i32>` object (that is, Vec is a generic type whose type argument is `i32`) on the stack that points to a collection of data on the heap. As you can see, you can access a vector's contents using `[]` like most languages. Rust has a neat syntax for declaring iterators with `..`. `0..5` creates an iterator that supplies values [0, 5) (zero inclusive to 5 exclusive). Using the `for x in y` syntax loops over values of y via x. An alternative to this would be to use `for num in &nums`, and `num` would have the value each iteration, rather than the index.

Add this code after the `main()` function:
```rust
fn find_max_index(nums: Vec<i32>) -> usize {
    let mut max_index = 0;

    for i in 0..nums.len() {
        if nums[i] > nums[max_index] {
            max_index = i;
        }
    }

    max_index
}
```
 
Add this code after the printing loop in `main()`:
```rust
    let max_index = find_max_index(nums);
    println!("max: {}", nums[max_index]);
```

If you try to `cargo run`, you'll get this compiler error:

    Compiling hello v0.1.0 (file:///Users/alexbowen/Code/Personal/RustTutorial/hello)
    error[E0382]: use of moved value: `nums`
      --> src/main.rs:21:25
       |
    20 |     let max_index = find_max_index(nums);
       |                                    ---- value moved here
    21 |     println!("max: {}", nums[max_index]);
       |                         ^^^^ value used here after move
       |
       = note: move occurs because `nums` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    
    error: aborting due to previous error
    
    error: Could not compile `hello`.
    
    To learn more, run the command again with --verbose.

The error `use of moved value: nums` demonstrates three closely-related Rust concepts: ownership, borrowing and moving. First, let's fix our error. Add an `&` before `Vec<i32>` in our function signature, and another `&` b!efore `nums` when we call it. The repaired lines should look like:
```rust
    let max_index = find_max_index(&nums);
```
```rust
fn find_max_index(nums: &Vec<i32>) -> usize {
```

It should run now. So, what happened? In Rust, variable bindings are **owned** by functions, closures, or structs. This concept allows the compiler to know if more than one `mut`able binding to the same data is **owned** at any given time. When we didn't use the `&`, we told the compiler that we were **moving** the vector into that function, and would no longer need it beyond this point. That's why the compiler complained about `^^^^ value used here after move`. Using the `&` tells the compiler that the function is only **borrowing** the data. The importance of this will become far more clear when we discuss threads.

Let's backtrack a bit. I'm sure the function signature is pretty familiar, the pattern being `fn [name]([parameters structured as name: type]) -> [return type if not void]`. So, where's the `return` keyword? In this simple function, we don't need one. Rust, like most languages, has **expressions** which have a value and **statements** which do not. A function body in Rust is an **expression**, so simply having the binding name at the end of the expression supplies a value. The `return` keyword is generally only used for multiple or early returns.

Now, `find_max_index` is pretty annoying to use. I made it this way to illustrate the borrowing concept. Let's replace it with something more practical. To ease this transition, you can replace your whole *main.rs* file with:
```rust
fn main() {
    let nums = vec![4, 8, 15, 16, 23, 42];

    println!("max: {}", find_max(&nums));
}

fn find_max(nums: &Vec<i32>) -> i32 {
    let mut max = std::i32::MIN;

    for num in nums {
        if *num > max {
            max = *num
        }
    }

    max
}
```

That's more like it. `find_max` takes a `&Vec<i32>` and returns the maximum `i32` that it finds. Note the `*` in `if *num > max {`. I fibbed a bit when I said that `&` was for borrowing. It's actually a more generalized operator for taking **references**. As you can guess from the `*`, the semantics of pointers and references are similar in concept to the same things in C/C++. When we use the `for` loop with `nums` as the iterator, `num` is now a pointer to a value in `nums` each iteration, so we need to dereference it to get the value we're pointing to. 

What happens if we were to call `find_max` with an empty vector? With the current implementation, we'll get the minimum i32...sounds pretty dumb. We can do better using another Rusty concept. Here's another new *main.rs*:
```rust
fn main() {
    let nums = vec![4, 8, 15, 16, 23, 42];

    match find_max(&nums) {
        Ok(max) => println!("max: {}", max),
        Err(err) => println!("Error: {}", err),
    }

    match find_max(&Vec::<i32>::new()) {
        Ok(max) => println!("max: {}", max),
        Err(err) => println!("Error: {}", err),
    }
}

fn find_max(nums: &Vec<i32>) -> Result<i32, &str> {
    if nums.len() == 0 {
        return Err("Vector is empty!");
    }

    let mut max = std::i32::MIN;

    for num in nums {
        if *num > max {
            max = *num
        }
    }

    Ok(max)
}
```
There's a lot of new stuff here. Skip down to the new signature of `find_max`. We now return a `Result<i32, &str>`. The `Result<TV, TE>` is a core Rust `struct` that allows the possibility of failure to be baked right into the type system. Anything in the standard library and anything in a well-implemented crate that can fail (network IO, parsing, etc...) will give you a `Result` with a type for its value and a type for its error. In our case, we return an `i32` if we have values, and a `&str` for any error condition. `Ok` and `Err` are enumerations supplied by the `Result` struct that let us easily return one or the other. As promised, the `return` keyword finally shows up here. `if` constructs are expressions themselves, so excluding the `return` would provide a value for the `if` expression, not the function.

Scroll back up to the `main()` function. What's this `match` all about? Rust does not have a switch-case construct, and `match` is like a switch on steroids. There's actually a pretty robust matching syntax in Rust that we won't have time to cover that applies both to the `match` construct and type requirements in functions. For now, just know that the basic idea is:

    match [something] {
        [value pattern] => use the value
    }

If you take a look at the first `match`, we're matching against the value returned by our `find_max` function. The first line of the body is:
```rust
        Ok(max) => println!("max: {}", max),
```
The left side of the `=>` says, "if the value of the result is Ok, give me a variable called max with the value". The right side of `=>` is executed if we succesfully match, and has access to the `max` variable. The next line is:

```rust
        Err(err) => println!("Error: {}", err),
```

The concept here is exactly the same, only for the error message. Since we hardcoded the `Vec` that we passed into `find_max`, I duplicated it all with an empty array to illustrate the usage of `Result`s and `match`. If you run your code now, you should see something like:

    bash-3.2$ cargo run
       Compiling hello v0.1.0 (file:///Users/alexbowen/Code/Personal/RustTutorial/hello)
        Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
         Running `target/debug/hello`
    max: 42
    Error: Vector is empty!
    bash-3.2$

Add this function:
```rust
fn first_even(nums: &Vec<i32>) -> Option<i32> {
    for num in nums {
        if *num % 2 == 0 {
            return Some(*num);
        }
    }

    None
}
```

Add this to the end of the `main()` function:
```rust
    match first_even(&nums) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }

    let odd_nums = vec![1, 3, 5, 7, 9, 11];
    match first_even(&odd_nums) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }
```

Here, we have another thing very closely related to the `Result` called the `Option`, which should by familiar to any Swift developers. `first_even` may be given a vector that has no even numbers, so it gives us `None` in that case. Where `Result`s are used to bake failure into the type system, `Option`s do the same for valid presence or absense of a value in a way far more robust and obvious than returning `null`, while also distinguishing `null` as a valid value when necessary. It's not uncommon to see `Result<Opton<TV>, TE>` or `Option<Result<TV, TE>>` when appropriate.

Rust has integrated syntactical support for tuples:
```rust
    let tuple = (1, 'a', "tuple", 15.35);
    println!("tuple 0: {}", tuple.0);
    println!("tuple 1: {}", tuple.1);
    println!("tuple 2: {}", tuple.2);
    println!("tuple 3: {}", tuple.3);
```
You can mix and match any type you'd like in a tuple. Remember when I said that **expressions** all have a value? An **expression**'s default value is `()`, an empty tuple. Like many moden languages, tuples can be destructured:
```rust
    let (int, character, float) = (12, 'b', 1.23);
    println!("int: {}", int);
    println!("character: {}", character);
    println!("float: {}", float);
```
Using destructuring, you can effectively return an arbitrary number of values from a function. Here's a modified `first_even` (without error handling):
```rust
fn first_even_with_index(nums: &Vec<i32>) -> (i32, usize) {
    for i in 0..nums.len() {
        if nums[i] % 2 == 0 {
            return (nums[i], i);
        }
    }

    (0, 0)
}
```
Call it like this:
```rust
    let (value, index) = first_even_with_index(&nums);
    println!("first even: {} at index {}", value, index);
```
Rust has good support for closures and functional programming. The easiest way to show these is by showing off iterator **consumers**, which is Rust's term for higher-order functions that operate on collections (kind of like Linq for any .NET familiars). Let's use these concepts to rework `first_even` again:
```rust
    match nums.iter().find(|x| *x % 2 == 0) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }
```
Calling `.iter()` on our `nums` vector gives us an iterator. `find()` is a higher-order function that returns an `Option` with the first element of `nums` matching the lambda expression. Let's try a more complicated lambda:
```rust
fn find_nth_even(nums: &Vec<i32>, n: i32) -> Option<&i32> {
    let mut num_found = -1;
    nums.iter().find(|x| {
        if **x % 2 == 0 {
            num_found = num_found + 1;
        }

        num_found == n
    })
}
```
This illustrates the syntax a bit better. The first closure we saw was a **predicate**, which only had the single line with the single expression. The second one we saw was a lot more like a function, but with `|` around the argument list instead of `(` and `)`. Since the two closures we saw were arguments to another function whose signature says exactly what it wants, we didn't need to provide the type hint. If we want a variable binding to a closure, we can do that like this:
```rust
fn find_nth_even(nums: &Vec<i32>, n: i32) -> Option<&i32> {
    let mut num_found = -1;
    let nth_closure = |x: &&i32| -> bool {
        if **x % 2 == 0 {
            num_found = num_found + 1;
        }

        num_found == n
    };

    nums.iter().find(nth_closure)
}
```
Note that there is a high degree of indirection here. Also note that both `n` and `num_found` are accessible in the lambda, since they were **captured**. Eventually, we'll want to use the `move` keyword before the arguments list to make the closure fully take ownership of things it captures, rather than borrow them. For now, it doesn't matter, since the `i32`s we're dealing with implement the `copy` **trait**. 

Wait, what's **trait**? For that, we'll first talk about `struct`s. By this point, you may want to go ahead and start a new Rust project if your *main.rs* from *hello* is geting messy. All you need to do is `cd` up out of the *hello* package, and `cargo new [package name] --bin`. Copy this over your new *main.rs*:
```rust
fn main() {
    let fred = Person {
        first_name: "Fred".to_string(),
        last_name: "Sanford".to_string(),
        birth_year: 1908,
    };
}

struct Person {
    first_name: String,
    last_name: String,
    birth_year: i16,
}
```
You'll get a bunch of compiler warnings about thigs not being done, and running it won't produce any output. `struct`s in Rust are very similar (at first) to `struct`s in Go, in the sense that definition is minimal, `struct` methods look more like extension methods from other langauges, and there is no language-specific pattern for constructors. Note that our `sturct` is called `Person` with a capitol "P". `struct`s in Rust are one of the few things that are typically CamelCased. In our `main()`, we allocate a new `Person` on the stack and assign to its fields. For now, ignore the `.to_string()` call for the literals. `String`s in Rust are pretty complicated, and I won't go too deep in this tutorial.

Let's actually do something. Add this to the end of your file:
```rust
impl Person {
    fn get_age(&self) -> i16 {
        2017 - self.birth_year
    }
}
```
Call it at the end of `main()`:
```rust
    println!("Age: {}", fred.get_age());
```

`get_age` subtracts the `Person`'s birth year from 2017 to get their age. It takes an immutable reference to self as the first argument. Rust knows you're implementing a method for `Person` because it's in the `impl` block and you're using the `self` keyword, so there's no need to be as verbose as to give the type of `self`. Note that I said it takes an *immutable* reference to `self`. We'll need a `mut`able reference to change anything. Add this inside the `impl` block:
```rust
    fn set_age(&mut self, age: i16) {
        self.birth_year = 2017 - age;
    }
```
First, notice that we said we need a `&mut self`, since we intend to mutate a property. If you just add this function, it will compile. However, if you try adding, say, `fred.set_age(25)`, you'll get an error because our `fred` binding was not declared as `mut`able (`let mut fred = ...`).

So, what's all this about **trait**s, then? **Trait**s are basically like interfaces, and are the closest thing Rust has to polymorphism. **Trait**s are a massive simplification and unification on what has become the incredibly complicated topic of Object-Oriented programming. Let's start off by implementing one of the many **trait**s from the standard library. Add this to the end of your file:
```rust
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}, age {}", self.first_name, self.last_name, self.get_age())
    }
}
```
The `display` **trait** is used to populate the `{}` in format strings. You can now `println!("{}", fred);` in `main()`. Let's make our own trait. Add this after `main()`:
```rust
fn print_name(nameable: &HasName) {
    println!("{}", nameable.get_name());
}

trait HasName {
    fn get_name(&self) -> String;
}
```

Add this after your `Person` definition:
```rust
impl HasName for Person {
    fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
```

Now you can call `print_name(&fred)` in `main()`. If you added another struct, say:
```rust
```