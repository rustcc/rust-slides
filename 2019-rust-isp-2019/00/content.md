# Lecture 01: Hello, Rust! #

![](img/rust.png)
Based on: [CIS 198 slides](https://github.com/cis198-2016s/slides)

Artyom Pavlov, 2019.

---
## Overview ##


"Rust is a systems programming language that runs blazingly fast, prevents
nearly all segfaults, and guarantees thread safety." &ndash;
[prev.rust-lang.org](https://prev.rust-lang.org/)

"Empowering everyone to build reliable and efficient software. " &ndash;
[rust-lang.org](https://www.rust-lang.org/)

---
### What _is_ Rust? ###

Rust is:
- Fast
- Safe
- Functional
- Zero-cost
- Excellent tooling

---
### Fast ###

- Rust compiles to native code
- Rust has no garbage collector
- Most abstractions have zero cost
- Fine-grained control over lots of things
- Pay for exactly what you need...
- ...and pay for most of it at compile time

---
### Safe ###

- No null
- No uninitialized memory
- No dangling pointers
- No double free errors
- No manual memory management!

---
### Functional ###

- First-class functions
- Trait-based generics
- Algebraic datatypes
- Pattern matching

---
### Zero-Cost 100% Safe Abstractions ###

- Rust's defining feature
- Strict compile-time checks remove need for runtime
- Big concept: Ownership

---
### Tooling ###

- `cargo`: one of the best package managers/build systems in class. Say no to dependency hell and makefiles!
- Built-in benchmarks, tests and docs generator (`rustdoc`).
- `clippy`: A LOT of additionall lints.
- `rustfmt`: code formatting utility.
- Rust Language Server (RLS) for improving IDE experience.
- Many C/C++ tools are usable with Rust: GDB, LLDB, Valgrind, perf, gprof, etc.

---
### Release Model

- Rust has a new stable release every six weeks
- Additionally Rust has 3-year edition releases
- Nightly builds are available, well, nightly
- Current stable: Rust 1.31 (2018 edition)
- Train model:

Date | Stable | Beta | Nightly
--- | --- | --- | ---
2018-09-13 | 🚂 1.29 | 🚆 1.30 | 🚝 1.31
2016-10-25 | 🚆 1.30 | 🚝 1.31 | 🚈 1.32
2016-12-06 | 🚝 1.31 | 🚈 1.32 | 🚅 1.33

---
### Development

- Rust is led by the Rust Team, mostly at Mozilla Research.
- Very active community involvement - on GitHub, Reddit, irc.
    - [Rust Source](https://github.com/rust-lang/rust/)
    - [Rust Users Forum](https://users.rust-lang.org/)
    - [Rust Internals Forum](https://internals.rust-lang.org/)
    - [/r/rust](http://www.reddit.com/r/rust)

---
### Who Uses Rust?

![](img/rust-in-production.png)

More companies and organizations can be found in the [Friends of Rust](https://prev.rust-lang.org/en-US/friends.html) page.

---
### Big Rust Projects

- [Servo](https://github.com/servo/servo)
- [Piston](https://github.com/PistonDevelopers/piston)
- [MaidSafe](https://maidsafe.net/)
- [MIO](https://github.com/carllerche/mio) and [tokio](https://github.com/tokio-rs/tokio)
- [Redox](https://www.redox-os.org/)
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [lalrpop](https://github.com/nikomatsakis/lalrpop)
- [cargo](https://github.com/rust-lang/cargo)
- [Rust itself!](https://github.com/rust-lang/rust/)

---
## Administrivia

- 5 lectures with practical exercises.
- Attendance: 50% (10% per lecture)
- Final project: 50%
- Passing grade: 60%
- Final project can be done individual or in a group (up to 5 people).
- You can ask questions in the Canvas chat or personally (ISR Lab, room 151).
- Class source material generally hosted on [GitHub](https://github.com/newpavlov/rust-isp-2019).
    - Corrections are welcomed via pull request/issue!
- Course is in development - give us feedback!

---
### Helpful Links ##

- [Official Rust Docs](https://doc.rust-lang.org/stable/std/)
- [The Rust Book (our course textbook)](https://doc.rust-lang.org/stable/book/)
- [Rust By Example](http://rustbyexample.com/)
- [Rust Playpen](https://play.rust-lang.org/)
    - Online editing and execution!
- [rust.godbolt.org](https://rust.godbolt.org/)
    - Examine generated assembly!

---
## Let's Dive In! ##

Hello, Rust!

```rust
fn main() {
    println!("Hello, world!");
}
```
- All code blocks have links to the Rust playpen so you can run them!

---
# Basic Rust Syntax #

---
## Variable Bindings ###
- Variables are bound with `let`:
```rust
let x = 17;
```

- Bindings are implicitly-typed: the compiler infers based on context.
- The compiler can't always determine the type of a variable, so sometimes you
  have to add type annotations.
```rust
let x: i16 = 17;
```

- Variables are inherently immutable:
```rust
let x = 5;
x += 1; // error: re-assignment of immutable variable x
let mut y = 5;
y += 1; // OK!
```

---
## Variable Bindings ###
- Bindings may be shadowed:
```rust
let x = 17;
let y = 53;
let x = "Shadowed!";
// x is not mutable, but we're able to re-bind it
```

- The shadowed binding for `x` above lasts until it goes out of scope.
- Above, we've effectively lost the first binding, since both `x`s are in the same scope.

- Patterns may also be used to declare variables:
```rust
let (a, b) = ("foo", 12);
let [c, d] = [1, 2];
```

---
## Expressions

- (Almost!) everything is an expression: something which returns a value.
    - Exception: variable bindings are not expressions.
- The "nothing" type is called "unit", which is written `()`.
    - The _type_ `()` has only one value: `()`.
    - `()` is the default return type.
- Discard an expression's value by appending a semicolon. Now it returns `()`.
    - Hence, if a function ends in a semicolon, it returns `()`.

```rust
fn foo() -> i32 { 5 }
fn bar() -> () { () }
fn baz() -> () { 5; }
fn qux()       { 5; }
```

---
## Expressions ###
- Because everything is an expression, we can bind many things to variable names:
```rust
let x = -5;
let y = if x > 0 { "greater" } else { "less" };
println!("x = {} is {} than zero", x, y);
```

- Side note: `"{}"` is Rust's (most basic) string interpolation operator
    - Similar to Python, Ruby, C#, and others; like `printf`'s `"%s"` in C/C++.
    - `"{:?}"` can be used for debug formatting
    - More information: [doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/)

---
## Comments

```rust
//! Comments like this are for module/crate documentation.

/// Triple-slash comments are docstring comments.
///
/// `rustdoc` uses docstring comments to generate
/// documentation, and supports **Markdown** formatting.
fn foo() {
    // Double-slash comments are normal.

    /* Block comments
    also exist /* and can be nested! */
     */
}
```

---
## Types

---
### Primitive Types ###

- `bool`: spelled `true` and `false`.
- `char`: spelled like `'c'` or `'😺'` (`char`s are Unicode code-points, i.e. 4 bytes long!).

- Numerics: specify the signedness and size.
    - `i8`, `i16`, `i32`, `i64`, `isize`
    - `u8`, `u16`, `u32`, `u64`, `usize`
    - `f32`, `f64`
    - `isize` & `usize` are the size of pointers (and therefore have
        machine-dependent size)
    - Literals are spelled like `10i8`, `10u16`, `10.0f32`, `10usize`.
    - Type inference for non-specific literals default to `i32` or `f64`:
      - e.g. `10` defaults to `i32`, `10.0` defaults to `f64`.

- Arrays, slices, `str`, tuples.
- Functions.

---
### Arrays ###
- Arrays are generically of type `[T; N]`.
    - N is a compile-time _constant_. Arrays cannot be resized.
    - Array access is bounds-checked at runtime.
    - No const generics yet. (planned to be added in 2019)
- Arrays are indexed with `[]` like most other languages:
    - `arr[3]` gives you the 4th element of `arr`

```rust
let arr1 = [1, 2, 3]; // (array of 3 elements)
let arr2 = [2; 32];   // (array of 32 `2`s)
```

---
### Slices ###
- Generically of type `&[T]`
- A "view" into an array (or heap allocated data) by reference
- Not created directly, but are borrowed from other variables
- Mutable `&mut [T]` or immutable `&[T]`
- How do you know when a slice is still valid? Coming soon...

```rust
let arr = [0, 1, 2, 3, 4, 5];
let val = arr[0];               // val = 0
let total_slice = &arr;         // Slice all of `arr`
let total_slice = &arr[..];     // Same, but more explicit
let partial_slice = &arr[2..5]; // [2, 3, 4]
```

---
### Strings ###
- Two types of Rust strings: `String` and `&str`.
- `String` is a heap-allocated, growable vector of characters.
- `&str` is a type&sup1; that's used to slice into `String`s.
- String literals like `"foo"` are of type `&str`.
- Strings are guaranteed to be encoded using UTF-8

```rust
let s: &str = "galaxy";
let s2: String = "галактика".to_string();
let s3: String = String::from("銀漢");
let s4: &str = &s3;
```

```rust
let s1 = "foobar";
// You can slice strings:
let s2 = &s1[1..3];
// But you can't index with []
// you can use `s1.chars().nth(1).unwrap()` instead
let s3 = s1[1] // does not work!
```

&sup1;`str` is an unsized type, which doesn't have a compile-time known size,
and therefore cannot exist by itself.

---
### Tuples ###
- Fixed-size, ordered, heterogeneous lists
- Index into tuples with `foo.0`, `foo.1`, etc.
- Can be destructured for example in `let` bindings

```rust
let foo: (i32, char, f64) = (72, 'H', 5.1);
let (x, y, z) = (72, 'H', 5.1);
let (a, b, c) = foo; // a = 72, b = 'H', c = 5.1
```

---
### Casting ###

- Cast between types with `as`:

```rust
let x: i32 = 100;
let y: u32 = x as u32;
```

- Naturally, you can only cast between types that are safe to cast between.
    - No casting `[i16; 4]` to `char`! (This is called a "non-scalar" cast)
    - There are unsafe mechanisms to overcome this, if you know what you're doing.

---
### `Vec<T>` ###

- A standard library type: you don't need to import anything.
- A `Vec` (read "vector") is a heap-allocated growable array.
    - (cf. Java's `ArrayList`, C++'s `std::vector`, etc.)
- `<T>` denotes a generic type.
    - The type of a `Vec` of `i32`s is `Vec<i32>`.
- Create `Vec`s with `Vec::new()` or the `vec!` macro.
    - `Vec::new()` is an example of namespacing. `new` is a function defined for
      the `Vec` struct.

---
### `Vec<T>` ###
```rust
// Explicit typing
let v0: Vec<i32> = Vec::new();

// v1 and v2 are equal
let mut v1 = Vec::new();
v1.push(1);
v1.push(2);
v1.push(3);

let v2 = vec![1, 2, 3];
```

```rust
// v3 and v4 are equal
let v3 = vec![0; 4];
let v4 = vec![0, 0, 0, 0];
```

---
### `Vec<T>` ###

```rust
let v2 = vec![1, 2, 3];
let x = v2[2]; // 3
```

- Like arrays and slices, vectors can be indexed and sliced with `[]`.
    - You can't index a vector with an i32/i64/etc.
    - You must use a `usize` because `usize` is guaranteed to be the same size as a pointer.
    - Other integers can be cast to `usize`:
      ```rust
      let i: i8 = 2;
      let y = v2[i as usize];
      ```

- Vectors has an extensive stdlib method list, which can be found in the
  [offical Rust documentation](https://doc.rust-lang.org/stable/std/vec/).

---
### References ###

- Reference *types* are written with an `&`: `&i32`.
- References can be taken with `&` (like C/C++).
- References can be _dereferenced_ with `*` (like C/C++).
- References are guaranteed to be valid.
    - Validity is enforced through compile-time checks!
- These are *not* the same as pointers!
- Reference lifetimes are pretty complex, as we'll explore later on in the course.

```rust
let x = 12;
let ref_x = &x;
println!("{}", *ref_x); // 12
```

---
## Control Flow ##

---
### If Statements ###

```rust
if x > 0 {
    10
} else if x == 0 {
    0
} else {
    println!("Not greater than zero!");
    -10
}
```
- No parens necessary.
- Entire if statement evaluates to one expression, so every arm must end with
  an expression of the same type.
    - That type can be unit `()`:

```rust
if x <= 0 {
    println!("Too small!");
}
```

---
### Loops ###
- Loops come in three flavors: `while`, `loop`, and `for`.
    - `break` and `continue` exist just like in most languages

- `while` works just like you'd expect:

```rust
let mut x = 0;
while x < 100 {
    x += 1;
    println!("x: {}", x);
}
```

---
### Loops ###
- `loop` is equivalent to `while true`, a common pattern.
    - Plus, the compiler can make optimizations knowing that it's infinite.

```rust
let mut x = 0;
loop {
    x += 1;
    println!("x: {}", x);
}
```

---
### Loops ###
- `for` is the most different from most C-like languages
     - `for` loops use an _iterator expression_:
     - `n..m` creates an iterator from n to m (exclusive).
     - Some data structures can be used as iterators, like arrays and `Vec`s.

```rust
// Loops from 0 to 9.
for x in 0..10 {
    println!("{}", x);
}

let xs = [0, 1, 2, 3, 4];
// Loop through elements in a slice of `xs`.
for x in &xs {
    println!("{}", x);
}
```

---
## Functions ##

```rust
fn foo(x: T, y: U, z: V) -> T {
    // ...
}
```

- `foo` is a function that takes three parameters:
    - `x` of type `T`
    - `y` of type `U`
    - `z` of type `V`
- `foo` returns a `T`.

- Must explicitly define argument and return types.
    - The compiler is actually smart enough to figure this out for you, but
      Rust's designers decided it was better practice to force explicit function
      typing.

---
### Functions

- The final expression in a function is its return value.
    - Use `return` for _early_ returns from a function.

```rust
fn square(n: i32) -> i32 {
    n * n
}

fn squareish(n: i32) -> i32 {
    if n < 5 { return n; }
    n * n
}

fn square_bad(n: i32) -> i32 {
    n * n;
}
```

- The last one won't even compile!
  - Why? It ends in a semicolon, so it evaluates to `()`.

---
### Function Objects ###

- Several things can be used as function objects:
    - Function pointers (a reference to a normal function)
    - Closures (covered later)
- Much more straightforward than C function pointers:

```rust
let x: fn(i32) -> i32 = square;
```

- Can be passed by reference:

```rust
fn apply_twice(f: &Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// ...

let y = apply_twice(&square, 5);
```

---
## Macros!

- Macros are like functions, but they're named with `!` at the end.
- Can do generally very powerful stuff.
    - They actually generate code at compile time!
- Call and use macros like functions.
- You can define your own with `macro_rules! macro_name` blocks.
    - These are *very* complicated (especially procedural macros).
    - We will not cover writing custom macros in this course.
- Because they're so powerful, a lot of common utilities are defined as macros.

---
### `print!` & `println!` ###
- Print stuff out. Yay.
- Use `{}` for general string interpolation, and `{:?}` for debug printing.
    - Some types can only be printed with `{:?}`, like arrays and `Vec`s.

```rust
print!("{}, {}, {}", "foo", 3, true);
// => foo, 3, true
println!("{:?}, {:?}", "foo", [1, 2, 3]);
// => "foo", [1, 2, 3]
```

---
### `format!` ###
- Uses `println!`-style string interpolation to create formatted `String`s.

```rust
let fmted = format!("{}, {:x}, {:?}", 12, 155, Some("Hello"));
// fmted == "12, 9b, Some("Hello")"
```

---
### `panic!(msg)`
- Exits current task with given message.
- Don't do this lightly! It is better to handle and report errors explicitly.

```rust
if x < 0 {
    panic!("Oh noes!");
}
```

---
### `assert!` & `assert_eq!`

- `assert!(condition)` panics if `condition` is `false`.
- `assert_eq!(left, right)` panics if `left != right`.
- `debug_assert!(condition)` and `debug_assert_eq!(left, right)` work in debug build, but omitted in release build.
- Useful for testing and catching illegal conditions.

```rust
#[test]
fn test_something() {
    let actual = 1 + 2;
    assert!(actual == 3);
    assert_eq!(3, actual);
}
```

---
### `unreachable!()`

- Used to indicate that some code should not be reached.
- `panic!`s when reached.
- Can be useful to track down unexpected bugs (e.g. optimization bugs).

```rust
if false {
    unreachable!();
}
```

---
### `unimplemented!()`

- Shorthand for `panic!("not yet implemented")`.

```rust
fn sum(x: Vec<i32>) -> i32 {
    // TODO
    unimplemented!();
}
```

---
### `dbg!()`

- A macro for quick and dirty debugging with which you can inspect the value of a given expression.
- Was added in Rust 1.32.

```rust
let a = 2;
let b = dbg!(a * 2) + 1;
//      ^-- prints: [src/main.rs:2] a * 2 = 4
assert_eq!(b, 5);
```

---
## Match statements ###
- `switch` on steroids.

```rust
let x = 3;

match x {
    1 => println!("one fish"),  // <- comma required
    2 => {
        println!("two fish");
        println!("two fish");
    },  // <- comma optional when using braces
    // we can match several patterns in one arm
    3 | 4 => println!("three or four, dunno"),
    _ => println!("no fish for you"), // "otherwise" case
}
```

- `match` takes an expression (`x`) and branches on a list of `value => expression` statements.
- The entire match evaluates to one expression.
    - Like `if`, all arms must evaluate to the same type.
- `_` is commonly used as a catch-all (cf. Haskell, OCaml).

---
## Match statements ###
```rust
let x = 3;
let y = -3;
let q = 10;

let s = match (x, y) {
    (1, 1) => "one".to_string(),
    (2, j) => format!("two, {}", j),
    (_, 3) => "three".to_string(),
    (i, j) if i > 5 && j < 0 => "On guard!".to_string(),
    // NB: note that we do not test x == 10 here!
    (q, k) => format!("???: {} {}", q, k),
};
println!("{}", s);
```

- The matched expression can be any expression (l-value), including tuples and function calls.
    - Matches can bind variables. `_` is a throw-away variable name.
- You _must_ write an exhaustive match in order to compile.
- Use `if`-guards to constrain a match to certain conditions.
- Patterns can get very complex.

---
## Exercise ###

- Write two functions for calculating Fibonacci numbers.
- The first function should use `for` loop without recursion.
- And the second one should use recursion and `match`.
- Reminder: Fibonacci numbers are calculated as `F(0) = 0; F(1) = 1; F(n) = F(n - 1) + F(n - 2)`
- You can use Rust Playground for this exercise.


---
## Exercise: non-recursive solution###
```rust
fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
	let mut sum = 1;
	let mut last = 0;
	let mut curr = 1;
	for _ in 1..n {
		sum = last + curr;
		last = curr;
		curr = sum;
	}
	sum
}
```

---
## Exercise: recursive solution###
```rust
fn fibonacci(n: u32) -> u64 {
	match n {
		0     => 0,
		1 | 2 => 1,
		3     => 2,
		// 50    => 12586269025,
		_     => fibonacci(n - 1) + fibonacci(n - 2),
	}
}
```

---
# Rust Environment & Tools

---
## Rustc ##

- Rust's compiler is `rustc`.
- Run `rustc your_program.rs` to compile into an executable `your_program`.
    - Things like warnings are enabled by default.
    - Read all of the output! It may be verbose but it is *very* useful.
- `rustc` doesn't need to be called once for each file like in C.
    - The build dependency tree is inferred from module declarations in the
      Rust code (starting at `main.rs` or `lib.rs`).
- Typically, you'll instead use `cargo`, Rust's package manager and build system.

---
## Cargo ##

- Rust's package manager & build tool
- Create a new project:
    - `cargo new project_name` (executable)
    - `cargo new project_name --lib` (library)
- Build your project: `cargo build`
- Run your project: `cargo run`
- Use `--release` flag to enable release build profile (longer compilation times, but more performant binaries)
- Typecheck code without building it: `cargo check` (much faster)
- Run your benchmarks if you have any: `cargo bench` (requires Nightly toolchain)
- Run your tests: `cargo test`

---
### Cargo.toml ###

- Cargo uses the `Cargo.toml` file to declare and manage dependencies and
  project metadata.
    - TOML is a simple format similar to INI.

```toml
[package]
name = "my_cool_project"
version = "0.1.0"
authors = ["My name"]
edition = "2018"

[dependencies]
uuid = "0.1"
rand = "0.3"

[profile.release]
opt-level = 3
debug = false
```

---
### `cargo test`

- A test is any function annotated with `#[test]`.
- `cargo test` will run all annotated functions in your project.
- Any function which executes without crashing (`panic!`ing) succeeds.
- Use `assert!` (or `assert_eq!`) to check conditions (and `panic!` on failure)

```rust
#[test]
fn it_works() {
    // ...
}
```


---
## Ownership & Borrowing

- Explicit ownership is the biggest new feature that Rust brings to the table!
- Ownership is all (well, mostly) checked at compile time!
- Newcomers to Rust often find themselves "fighting with the borrow checker"
   trying to get their code to compile
- The ownership model is the biggest thing that Rust brings to the table, its
  claim to fame.
- Ownership is something that's checked at compile time and has as little
  runtime cost as possible.
- So it's zero (or very little) runtime cost, but you pay for it with a longer
  compilation time and learning curve. Which is where the phrase "fighitng with
  the borrow checker" comes from, when you have to work around the compiler's
  restrictions to figure out how to do what you want.

---
## Ownership

- A variable binding _takes ownership_ of its data.
    - A piece of data can only have one owner at a time.
- When a binding goes out of scope, the bound data is released automatically.
    - For heap-allocated data, this means de-allocation.
- Data _must be guaranteed_ to outlive its references.

```rust
fn foo() {
    // Creates a Vec object.
    // Gives ownership of the Vec object to v1.
    let mut v1 = vec![1, 2, 3];

    v1.pop();
    v1.push(4);

    // At the end of the scope, v1 goes out of scope.
    // v1 still owns the Vec object, so it can be cleaned up.
}
```

---
## Ownership

So here are the basics.
- When you introduce a variable binding, it takes ownership of its data. And a
  piece of data can only have one owner at a time.
- When a variable binding goes out of scope, nothing has access to the data
  anymore, so it can be released. Which means, if it's on the heap, it can be
  de-allocated.
- And data must be guaranteed to outlive its references. Or, all references are
  guaranteed to be valid.

---
## Move Semantics

```rust
let v1 = vec![1, 2, 3];

// Ownership of the Vec object moves to v2.
let v2 = v1;

println!("{}", v1[2]); // error: use of moved value `v1`
```

- `let v2 = v1;`
    - We don't want to copy the data, since that's expensive.
    - The data cannot have multiple owners.
    - Solution: move the Vec's ownership into `v2`, and declare `v1` invalid.
- `println!("{}", v1[2]);`
    - We know that `v1` is no longer a valid variable binding, so this is an error.
- Rust can reason about this at compile time, so it throws a compiler error.

---
## Move Semantics

- Moving ownership copies data. BUT:
    - Often moves are optimized out by compiler.
    - When we move ownership of a heap allocated data (e.g. `Vec<T>`), we do
        not touch data on heap, just few bytes allocated on stack are copied
        (pointer to heap, length and capacity; 24 bytes on 64-bit machine)
- Moves are automatic (via assignments); no need to use something like C++'s
  `std::move`.
    - However, there are functions like `std::mem::replace` in Rust to provide
      advanced ownership management.
- For more finer-grained control see standrard library functions:
`std::mem::replace`, `std::mem::swap` and others.

---
## Ownership

- Ownership does not always have to be moved.
- What would happen if it did? Rust would get very tedious to write:

```rust
fn vector_length(v: Vec<i32>) -> Vec<i32> {
    // Do whatever here,
    // then return ownership of `v` back to the caller
}
```
- You could imagine that this does not scale well either.
    - The more variables you had to hand back, the longer your return type would be!
    - Imagine having to pass ownership around for 5+ variables at a time :(

---
## Borrowing

- Instead of transferring ownership, we can _borrow_ data.
- A variable's data can be borrowed by taking a reference to the variable;
  ownership doesn't change.
    - When a reference goes out of scope, the borrow is over.
    - The original variable retains ownership throughout.

```rust
let v = vec![1, 2, 3];

// v_ref is a reference to v.
let v_ref = &v;

// use v_ref to access the data in the vector v.
assert_eq!(v[1], v_ref[1]);
```

---
## Borrowing

- Caveat: this adds restrictions to the original variable.
- Ownership cannot be transferred from a variable while references to it exist.
    - That would invalidate the reference.

```rust
let v = vec![1, 2, 3];

// v_ref is a reference to v.
let v_ref = &v;

// Moving ownership to v_new would invalidate v_ref.
// error: cannot move out of `v` because it is borrowed
let v_new = v;
```

---
## Borrowing

```rust
/// `length` only needs `vector` temporarily, so it is borrowed.
fn length(vec_ref: &Vec<i32>) -> usize {
    // vec_ref is auto-dereferenced when you call methods on it.
    vec_ref.len()
    // you can also explicitly dereference.
    // (*vec_ref).len()
}

fn main() {
    let vector = vec![];
    length(&vector);
    println!("{:?}", vector); // this is fine
}
```
- Note the type of `length`: `vec_ref` is passed by reference, so it's now an `&Vec<i32>`.
- References, like bindings, are *immutable* by default.
- The borrow is over after the reference goes out of scope (at the end of `length`).

---
## Borrowing

```rust
/// `push` needs to modify `vector` so it is borrowed mutably.
fn push(vec_ref: &mut Vec<i32>, x: i32) {
    vec_ref[0] = 100;
    vec_ref.push(x);
}

fn main() {
    let mut vector: Vec<i32> = vec![1, 2];
    let vector_ref: &mut Vec<i32> = &mut vector;
    push(vector_ref, 4);
    assert_eq!(vector_ref, &[100, 2, 4]);
}
```
- Variables can be borrowed by _mutable_ reference: `&mut vec_ref`.
    - `vec_ref` is a reference to a mutable `Vec`.
    - The type is `&mut Vec<i32>`, not `&Vec<i32>`.
- Different from a reference which is variable.

---
## Borrowing

```rust
/// `push` needs to modify `vector` so it is borrowed mutably.
fn push2(vec_ref: &mut Vec<i32>, x: i32) {
    // error: cannot move out of borrowed content.
    let vector = *vec_ref;
    vector.push(x);
}

fn main() {
    let mut vector = vec![];
    push2(&mut vector, 4);
}
```
- Error! You can't dereference `vec_ref` into a variable binding because that
  would change the ownership of the data.

---
## Borrowing

- Rust will auto-dereference variables...
    - When making method calls on a reference.
    - When passing a reference as a function argument.

```rust
/// `length` only needs `vector` temporarily, so it is borrowed.
fn length(vec_ref: &&Vec<i32>) -> usize {
    // vec_ref is auto-dereferenced when you call methods on it.
    vec_ref.len()
}

fn main() {
    let vector = vec![];
    length(&&&&&&&&&&&&vector);
}
```

---
## Borrowing

- You will have to dereference variables...
    - When writing into them.
    - And other times that usage may be ambiguous.

```rust
let mut a = 5;
let ref_a = &mut a;
*ref_a = 4;
println!("{}", *ref_a + 4);
// ==> 8
```

---
## `Copy` Types

- Rust defines a trait&sup1; named `Copy` that signifies that a type may be
    copied instead whenever it would be moved.
- Most primitive types are `Copy` (`i32`, `f64`, `char`, `bool`, etc.)
- Types that contain references may not be `Copy` (e.g. `Vec`, `String`).
```rust
let x: i32 = 12;
let y = x; // `i32` is `Copy`, so it's not moved :D
println!("x still works: {}, and so does y: {}", x, y);
```

&sup1; Like a Java/Go interface or Haskell typeclass


---
## Borrowing Rules
##### _The Holy Grail of Rust_
Learn these rules, and they will serve you well.

- You can't keep borrowing something after it stops existing.
- One object may have many immutable references to it (`&T`).
- **OR** _exactly one_ mutable reference (`&mut T`) (not both).
- That's it!

![](img/holy-grail.jpg)

---
### Borrowing Prevents...

- Iterator invalidation due to mutating a collection you're iterating over.
- This pattern can be written in C, C++, Java, Python, Javascript...
    - But may result in, e.g, `ConcurrentModificationException` (at runtime!)

```rust
let mut vs = vec![1,2,3,4];
for v in &vs {
    vs.pop();
    // ERROR: cannot borrow `vs` as mutable because
    // it is also borrowed as immutable
}
```

- `pop` needs to borrow `vs` as mutable in order to modify the data.
- But `vs` is being borrowed as immutable by the loop!

---
### Borrowing Prevents...

- Use-after-free
- Valid in C, C++...

```rust
let y: &i32;
{
    let x = 5;
    y = &x; // error: `x` does not live long enough
}
println!("{}", *y);
```

- The full error message:

```
error: `x` does not live long enough
note: reference must be valid for the block suffix following statement
    0 at 1:16
...but borrowed value is only valid for the block suffix
    following statement 0 at 4:18
```

- This eliminates a _huge_ number of memory safety bugs _at compile time_.
- As a side note, this technique of creating a block to limit the scope of a
variable (in this case x) is pretty useful.

---
### Borrowing Prevents...

- Data races in multithreaded environments.
- It checks at compile time if it's safe to share or send a given piece of data.
- Compiler ensures that programm uses necessary synchronization using various primitives: mutexes, atomics, channels, etc.
- NB: data races != race condition.
- Check out TRPL section of ["Fearless Concurrency"](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---
## Example: Vectors

- You can iterate over `Vec`s in three different ways:

```rust
let mut vs = vec![0,1,2,3,4,5,6];

// Borrow immutably
for v in &vs { // Can also write `for v in vs.iter()`
    println!("I'm borrowing {}.", v);
}

// Borrow mutably
for v in &mut vs { // Can also write `for v in vs.iter_mut()`
    *v = *v + 1;
    println!("I'm mutably borrowing {}.", v);
}

// Take ownership of the whole vector
for v in vs { // Can also write `for v in vs.into_iter()`
    println!("I now own {}! AHAHAHAHA!", v);
}

// `vs` is no longer valid
```


---
## Structured Data

- Rust has two simple ways of creating structured data types:
    - Structs: C-like structs to hold data.
    - Enums: OCaml-like; data that can be one of several types.

- Structs and enums may have one or more implementation blocks (`impl`s) which
  define methods for the data type.

---
## Structs

- A struct declaration:
    - Fields are declared with `name: type`.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

- By convention, structs have `CamelCase` names, and their fields have `snake_case` names.
- Structs may be instantiated with fields assigned in braces.

```rust
let origin = Point { x: 0, y: 0 };
```

---
## Structs

- Struct fields may be accessed with dot notation.
- Structs may not be partially-initialized.
    - You must assign all fields upon creation, or declare an uninitialized
      struct that you initialize later.

```rust
let mut p = Point { x: 19, y: 8 };
p.x += 1;
p.y -= 1;
```

---
## Structs

- Structs do not have field-level mutability.
- Mutability is a property of the **variable binding**, not the type.
- Field-level mutability (interior mutability) can be achieved via `Cell` types.
    - More on these very soon.

```rust
struct Point {
    x: i32,
    mut y: i32, // Illegal!
}
```

---
## Structs

- Structs are namespaced with their module name.
    - The fully qualified name of `Point` is `foo::Point`.
- Struct fields are private by default.
    - They may be made public with the `pub` keyword.
- Private fields may only be accessed from within the module where the struct is
    declared.

```rust
mod foo {
    pub struct Point {
        pub x: i32,
        y: i32,
    }
}

fn main() {
    let b = foo::Point { x: 12, y: 12 };
    //      ^~~~~~~~~~~~~~~~~~~~~~~~~~~
    // error: field `y` of struct `foo::Point` is private
}
```

---
## Structs

```rust
mod foo {
    pub struct Point {
        pub x: i32,
        y: i32,
    }

    // Creates and returns a new point
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}
```

- `new` is inside the same module as `Point`, so accessing private fields is
  allowed.

---
### Struct `match`ing

- Destructure structs with `match` statements.

```rust
pub struct Point {
    x: i32,
    y: i32,
}

match p {
    Point { x, y } => println!("({}, {})", x, y)
}
```

---
### Struct `match`ing

- Some other tricks for struct `match`es:

```rust
match p {
    Point { y: y1, x: x1 } => println!("({}, {})", x1, y1)
}

match p {
    Point { y, .. } => println!("{}", y)
}
```
- Fields do not need to be in order.
- List fields inside braces to bind struct members to those variable names.
    - Use `struct_field: new_var_binding` to change the variable it's bound to.
- Omit fields: use `..` to ignore all unnamed fields.

---
### Struct Update Syntax

- A struct initializer can contain `.. s` to copy some or all fields from `s`.
- Any fields you don't specify in the initializer get copied over from the target struct.
- The struct used must be of the same type as the target struct.
    - No copying same-type fields from different-type structs!

```rust
struct Foo { a: i32, b: i32, c: i32, d: i32, e: i32 }

let mut x = Foo { a: 1, b: 1, c: 2, d: 2, e: 3 };
let x2 = Foo { e: 4, .. x };

// Useful to update multiple fields of the same struct:
x = Foo { a: 2, b: 2, e: 2, .. x };
```

---
### Tuple Structs

- Variant on structs that has a name, but no named fields.
- Have numbered field accessors, like tuples (e.g. `x.0`, `x.1`, etc).
- Can also `match` these.

```rust
struct Color(i32, i32, i32);

let mut c = Color(0, 255, 255);
c.0 = 255;
match c {
    Color(r, g, b) => println!("({}, {}, {})", r, g, b)
}
```

---
### Tuple Structs

- Helpful if you want to create a new type that's not just an alias.
    - This is often referred to as the "newtype" pattern.
- These two types are structurally identical, but not equatable.

```rust
// Not equatable
struct Meters(i32);
struct Yards(i32);

// May be compared using `==`, added with `+`, etc.
type MetersAlias = i32;
type YardsAlias  = i32;
```

---
### Unit Structs (Zero-Sized Types)

- Structs can be declared to have zero size.
    - This struct has no fields!
- We can still instantiate it.
- It can be used as a "marker" type on other data structures.
    - Useful to indicate, e.g., the type of data a container is storing.

```rust
struct Unit;

let u = Unit;
```

---
## Enums

- An enum, or "sum type", is a way to express some data that may be one of several things.
- Much more powerful than in Java, C, C++, C#...
- Each enum variant can have:
    - no data (unit variant)
    - named data (struct variant)
    - unnamed ordered data (tuple variant)

```rust
enum Resultish {
    Ok,
    Warning { code: i32, message: String },
    Err(String)
}
```

---
## Enums

- Enum variants are namespaced by their enum type: `Resultish::Ok`.
    - You can import all variants with `use Resultish::*`.
- Enums, much as you'd expect, can be matched on like any other data type.

```rust
match make_request() {
    Resultish::Ok =>
        println!("Success!"),
    Resultish::Warning { code, message } =>
        println!("Warning: {}!", message),
    Resultish::Err(s) =>
        println!("Failed with error: {}", s),
}
```

---
## Enums

- Enum constructors like `Resultish::Ok` and the like can be used as functions.
- This is not currently very useful, but will become so when we cover closures &
    iterators.

---
## Recursive Types

- You might think to create a nice functional-style `List` type:

```rust
enum List {
    Nil,
    Cons(i32, List),
}
```

---
## Recursive Types

- Such a definition would have infinite size at compile time!
- Structs & enums are stored inline by default, so they may not be recursive.
    - i.e. elements are not stored by reference, unless explicitly specified.
- The compiler tells us how to fix this, but what's a `box`?

```rust
enum List {
    Nil,
    Cons(i32, List),
}
// error: invalid recursive enum type
// help: wrap the inner value in a box to make it representable
```

---
## Boxes, Briefly

- A `box` (lowercase) is a general term for one of Rust's ways of allocating data on the heap.
- A `Box<T>` (uppercase) is a heap pointer with exactly one owner.
    - A `Box` owns its data (the `T`) uniquely-- it can't be aliased.
- `Box`es are automatically destructed when they go out of scope.
- Create a `Box` with `Box::new()`:

```rust
let boxed_five = Box::new(5);

enum List {
    Nil,
    Cons(i32, Box<List>), // OK!
}
```
- We'll cover these in greater detail when we talk more about pointers.

---
## Methods

```rust
impl Point {
    pub fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.distance();
}
```

- Methods can be implemented for structs and enums in an `impl` block.
- Like fields, methods may be accessed via dot notation.
- Can be made public with `pub`.
    - `impl` blocks themselves don't need to be made `pub`.
- Work for enums in exactly the same way they do for structs.

---
## Methods

- The first argument to a method, named `self`, determines what kind of ownership the method
  requires.
- `&self`: the method *borrows* the value.
    - Use this unless you need a different ownership model.
- `&mut self`: the method *mutably borrows* the value.
    - The function needs to modify the struct it's called on.
- `self`: the method takes ownership.
    - The function consumes the value and may return something else.

---
## Methods

```rust
impl Point {
    fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn mirror_y(self) -> Point {
        Point { x: -self.x, y: self.y }
    }
}
```

- `distance` needs to access but not modify fields.
- `translate` modifies the struct fields.
- `mirror_y` returns an entirely new struct, consuming the old one.

---
## Associated Functions

```rust
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let p = Point::new(1, 2);
}
```

- Associated function: like a method, but does not take `self`.
    - This is called with namespacing syntax: `Point::new()`.
        - Not `Point.new()`.
    - Like a "static" method in Java.
- A constructor-like function is usually named `new`.
    - No inherent notion of constructors, no automatic construction.

---
## Implementations
- Methods, associated functions, and functions in general may not be overloaded.
    - e.g. `Vec::new()` and `Vec::with_capacity(capacity: usize)` are both
      constructors for `Vec`
- Methods may not be inherited.
    - Rust structs & enums must be composed instead.
    - However, traits (coming soon) cover some of the inheritence functionality.

---
## Patterns

- Use `..=` to specify a range of values. Useful for numerics and `char`s.
- You can encounter `...` which is deprecated variant of `..=`.
- Use `_` to bind against any value (like any variable binding) and discard the
  binding.

```rust
let x = 17;

match x {
    0 ..= 5 => println!("zero through five (inclusive)"),
    _ => println!("You still lose the game."),
}
```

---
### `match`: References

- Get a reference to a variable by asking for it with `ref`.

```rust
let x = 17;

match x {
    ref r => println!("Of type &i32: {}", r),
}
```

- And get a mutable reference with `ref mut`.
    - Only if the variable was declared `mut`.
- Note: with introduction of ["match ergonomics"](https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md) `ref` patterns become much less common.

```rust
let mut x = 17;

match x {
    ref r if x == 5 => println!("{}", r),
    ref mut r => *r = 5
}
```
- Similar to `let ref`.

---
### `if-let` Statements

- If you only need a single match arm, it often makes more sense to use Rust's `if-let` construct.
- For example, given the `Resultish` type we defined earlier:

```rust
enum Resultish {
    Ok,
    Warning { code: i32, message: String },
    Err(String),
}
```

---
### `if-let` Statements
- Suppose we want to report an error but do nothing on `Warning`s and `Ok`s.

```rust
match make_request() {
    Resultish::Err(_) => println!("Total and utter failure."),
    _ => println!("ok."),
}
```

- We can simplify this statement with an `if-let` binding:

```rust
let result = make_request();

if let Resultish::Err(s) = result {
    println!("Total and utter failure: {}", s);
} else {
    println!("ok.");
}
```

---
### `while-let` Statement

- There's also a similar `while-let` statement, which works like an `if-let`,
   but iterates until the condition fails to match.

```rust
while let Resultish::Err(s) = make_request() {
    println!("Total and utter failure: {}", s);
}
```

---
### Inner Bindings

- With more complicated data structures, use `@` to create variable bindings for
    inner elements.

```rust
#[derive(Debug)]
enum A { None, Some(B) }
#[derive(Debug)]
enum B { None, Some(i32) }

fn foo(x: A) {
    match x {
        a @ A::None              => println!("a is A::{:?}", a),
        ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
        A::Some(b @ B::Some(_))  => println!("b is B::{:?}", b),
    }
}

foo(A::None);             // ==> x is A::None
foo(A::Some(B::None));    // ==> a is A::Some(None)
foo(A::Some(B::Some(5))); // ==> b is B::Some(5)
```

---
## Lifetimes

- There's one more piece to the ownership puzzle: Lifetimes.
- Lifetimes generally have a pretty steep learning curve.
  - We may cover them again later on in the course under a broader scope if
      necessary.
- Don't worry if you don't understand these right away.

---
## Lifetimes

- Imagine This:
  1. I acquire a resource.
  2. I lend you a reference to my resource.
  3. I decide that I'm done with the resource, so I deallocate it.
  4. You still hold a reference to the resource, and decide to use it.
  5. You crash 😿.
- We've already said that Rust makes this scenario impossible, but glossed over
    how.
- We need to prove to the compiler that _step 3_ will never happen before _step 4_.

---
## Lifetimes

- Ordinarily, references have an implicit lifetime that we don't need to care
    about:
```rust
fn foo(x: &i32) {
    // ...
}
```
- However, we can explicitly provide one instead:
```rust
fn bar<'a>(x: &'a i32) {
    // ...
}
```

- `'a`, pronounced "tick-a" or "the lifetime *a*" is a *named* lifetime
  parameter.
    - `<'a>` declares generic parameters, including lifetime parameters.
    - The type `&'a i32` is a reference to an `i32` that lives at least as
      long as the lifetime `'a`.

???

## Stop here briefly to discuss

---
## Lifetimes

- The compiler is smart enough not to need `'a` above, but this isn't always the
  case.
- Scenarios that involve multiple references or returning references often
  require explicit lifetimes.
  - Speaking of which...

---
## Multiple Lifetime Parameters

```rust
fn borrow_x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str;
```

- In `borrow_x_or_y`, all input/output references all have the same lifetime.
    - `x` and `y` are borrowed (the reference is alive) as long as the returned
      reference exists.

```rust
fn borrow_p<'a, 'b>(p: &'a str, q: &'b str) -> &'a str;
```

- In `borrow_p`, the output reference has the same lifetime as `p`.
    - `q` has a separate lifetime with no constrained relationship to `p`.
    - `p` is borrowed as long as the returned reference exists.

---
## Lifetimes

- Okay, great, but what does this all mean?
    - If a reference `R` has a lifetime `'a`, it is _guaranteed_ that it will not
        outlive the owner of its underlying data (the value at `*R`)
    - If a reference `R` has a lifetime of `'a`, anything else with the lifetime
      `'a` is _guaranteed_ to live as long `R`.
- This will probably become more clear the more you use lifetimes yourself.

---
## Lifetimes - `struct`s

- Structs (and struct members) can have lifetime parameters.

```rust
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> {
    pizza: &'a Pizza,  // <- references in structs must
    index: u32,        //    ALWAYS have explicit lifetimes
}

let p1 = Pizza(vec![1, 2, 3, 4]);
{
    let s1 = PizzaSlice { pizza: &p1, index: 2 }; // this is okay
}

let s2;
{
    let p2 = Pizza(vec![1, 2, 3, 4]);
    s2 = PizzaSlice { pizza: &p2, index: 2 };
    // no good - why?
}
```

- Currently Rust does not support self-referential structs out-of-box.

---
## Lifetimes - `struct`s

- Lifetimes can be constrained to "outlive" others.
    - Same syntax as type constraint: `<'b: 'a>`.

```rust
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> { pizza: &'a Pizza, index: u32 }
struct PizzaConsumer<'a, 'b: 'a> { // says "b outlives a"
    slice: PizzaSlice<'a>, // <- currently eating this one
    pizza: &'b Pizza,      // <- so we can get more pizza
}

fn get_another_slice(c: &mut PizzaConsumer, index: u32) {
    c.slice = PizzaSlice { pizza: c.pizza, index: index };
}

let p = Pizza(vec![1, 2, 3, 4]);
{
    let s = PizzaSlice { pizza: &p, index: 1 };
    let mut c = PizzaConsumer { slice: s, pizza: &p };
    get_another_slice(&mut c, 2);
}
```

---
## Lifetimes - `'static`

- There is one reserved, special lifetime, named `'static`.
- `'static` means that a reference may be kept (and will be valid) for the
  lifetime of the entire program.
    - i.e. the data referred to will never go out of scope.
- All `&str` literals have the `'static` lifetime.

```rust
let s1: &str = "Hello";
let s2: &'static str = "World";
```

---
### Structured Data With Lifetimes

- Any struct or enum that contains a reference must have an explicit lifetime.
- Normal lifetime rules otherwise apply.

```rust
struct Foo<'a, 'b> {
  v: &'a Vec<i32>,
  s: &'b str,
}
```

---
### Lifetimes in `impl` Blocks

- Implementing methods on `Foo` struct requires lifetime annotations too!
- You can read this block as "the implementation using the lifetimes `'a` and
    `'b` for the struct `Foo` using the lifetimes `'a` and `'b`."

```rust
impl<'a, 'b> Foo<'a, 'b> {
  fn new(v: &'a Vec<i32>, s: &'b str) -> Foo<'a, 'b> {
    Foo {
      v: v,
      s: s,
    }
  }
}
```


---
## Exercises/homework
- Solve "Ownership", "Shared borrows", "Mutable borrows", "Structs and such" exercises from: http://www.rust-tutorials.com/exercises/
- Read The Rust Programming Language book: https://doc.rust-lang.org/book/
- Check examples in the Rust By Example book: https://doc.rust-lang.org/rust-by-example/
