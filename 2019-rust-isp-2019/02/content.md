# Generics & Traits

### Rust ISP 2019 Lecture 02
Based on: [CIS 198 slides](https://github.com/cis198-2016s/slides)

Artyom Pavlov, 2019.

---
## `&T` and `&mut T`

- Your basic, economy-class references.
- Zero runtime cost; all checks are done at compile time.
- Not allowed to outlive their associated lifetime.
    - Can introduce serious lifetime complexity if you're not careful!
- Use these unless you _actually_ need something more complicated.

---
## `Box<T>`

- A `Box<T>` is one of Rust's ways of allocating data on the heap.
- A `Box<T>` owns a `T`, so its pointer is unique - it can't be aliased (only referenced).
- `Box`es are automatically freed when they go out of scope.
- Almost the same as unboxed values, but dynamically allocated.
- Create a `Box` with `Box::new()`.

```rust
let boxed_five = Box::new(5);
```

---
## `Box<T>`

- Pros:
    - Easiest way to put something on the heap.
    - Zero-cost abstraction for dynamic allocation.
    - Shares typical borrowing and move semantics.
    - Automatic destruction.
- Cons (ish):
    - The `T` is strictly owned by the `Box` - only one owner.
        - This means the particular variable holding the box can't go away
          until all references are gone - sometimes this won't work out!

---
## Aside: Box Syntax & Patterns

- It's currently not possible to destructure the `Box` inside the `Option`. :(
- So you'll have to write code like this:

```rust
let opt_box: Option<Box<i32>> = Some(Box::new(5));

match opt_box {
    Some(boxed) => {
        let unboxed = *boxed;
        println!("Some {}", unboxed);
    }
    None => println!("None :("),
}

```

---
## Aside: Box Syntax & Patterns

- But in Nightly Rust, it's possible, thanks to `box` syntax!

```rust
#![feature(box_syntax, box_patterns)]

let opt_box = Some(box 5);

match opt_box {
    Some(box unboxed) => println!("Some {}", unboxed),
    None => println!("None :("),
}
```

- This may change before it reaches Stable.

---
## `std::rc::Rc<T>`

- Want to share a pointer with your friends? Use an `Rc<T>`!
- A "**R**eference **C**ounted" pointer.
    - Keeps track of how many aliases exist for the pointer.
- Call `clone()` on an `Rc` to get a reference.
    - Increments its reference count. No data gets copied!
- When the ref count drops to 0, the value is freed.
- The `T` can only be mutated when the reference count is 1 😕.
    - Same as the borrowing rules - there must be only one owner.

```rust
let mut shared = Rc::new(6);
{
    println!("{:?}", Rc::get_mut(&mut shared)); // ==> Some(6)
}
let mut cloned = shared.clone(); // ==> Another reference to same data
{
    println!("{:?}", Rc::get_mut(&mut shared)); // ==> None
    println!("{:?}", Rc::get_mut(&mut cloned)); // ==> None
}
```

---
## `std::rc::Arc<T>`

- Mostly equivalent to `Rc<T>`, but uses **A**tomic reference counting.
- Using atomics is a bit less performant than the usual integers, but allows
    to safely share data across threads.
- Compiler enforces usage of `Arc<T>` instead of `Rc<T>` when data is shared
    between threads. And it's done at compile time!

---
## `std::rc::Weak<T>`

- Reference counting has weaknesses: if a cycle is created:
    - A has an `Rc` to B, B has an `Rc` to A - both have count = 1.
    - They'll never be freed! ~~Eternal imprisonment~~ a memory leak!
- This can be avoided with _weak references_.
    - These don't increment the _strong reference_ count.
    - But that means they aren't always valid!
- An `Rc` can be downgraded into a `Weak` using `Rc::downgrade()`.
    - To access it, turn it back into `Rc`: `weak.upgrade() -> Option<Rc<T>>`
    - Nothing else can be done with `Weak` - upgrading prevents the value from becoming invalid mid-use.

---
## Strong Pointers vs. Weak Pointers

- When do you use an `Rc` vs. a `Weak`?
    - Generally, you probably want a strong reference via `Rc`.
- If your ownership semantics need to convey a notion of possible access to data but no
    ownership, you might want to use a `Weak`.
    - Such a structure would also need to be okay with the `Weak` coming up as
        `None` when upgraded.
- Any structure with reference cycles may also need `Weak`, to avoid the leak.
    - Note: `Rc` cycles are difficult to create in Rust, because of mutability rules.

---
## `std::rc::Rc<T>`

- Pros:
    - Allows sharing ownership of data.
- Cons:
    - Has a (small) runtime cost.
        - Holds two reference counts (strong and weak).
        - Must update and check reference counts dynamically.
    - Reference cycles can leak memory. This can only be resolved by:
        - Avoiding creating dangling cycles.
        - Garbage collection (which Rust doesn't have).

---
## Cells

- A way to wrap data to allow _interior mutability_.
- An _immutable_ reference allows modifying the contained value!
- Check documentation for [`std::cell`](https://doc.rust-lang.org/std/cell/index.html) module and types inside it.

---
## `*const T` & `*mut T`

- C-like raw pointers: they just point... somewhere in memory.
- No ownership rules.
- No lifetime rules.
- Zero-cost abstraction... because there is no abstraction.
- Requires `unsafe` to be dereferenced.
    - May eat your laundry if you're not careful.
- Use these if you're building a low-level structure like `Vec<T>` or for FFI,
    but not in typical code.
    - Can be useful for manually avoiding runtime costs.
- We won't get to unsafe Rust in this lecture, but for now:
    - Unsafe Rust is basically C with Rust syntax.
    - Unsafe means having to manually maintain Rust's assumptions
      (borrowing, non-nullability, non-undefined memory, correct data aligment, etc.)

---
## `const`

```rust
const PI: f32 = 3.1419;
```

- Defines constants that live for the duration of the program.
- Must annotate the type!
- Constants "live" for the duration of the program.
    - Think of them as being inlined every time they're used.
    - No guarantee that multiple references to the same constant are the same.

---
## `static`

```rust
static PI: f32 = 3.1419;
```

- As above: must annotate type.
- Typical global variable with fixed memory address.
- All references to static variables has the `'static` lifetime, because statics
  live as long as the program.
- `unsafe` to mutate.

```rust
let life_of_pi: &'static f32 = &PI;
```

- String literals are references (with lifetime `'static`) to `static str`s.

---
## `static`

```rust
static mut counter: i32 = 0;
```

- You can create mutable static variables, but you can only mutate them inside
  `unsafe` blocks.
    - Rust forces you to declare when you're doing things that are...
      ~~morally questionable~~ potentially going to crash your program.

---
# Modules & Crates

---
## Modules

- We've seen these in the homework, but not talked about them.
- Everything in Rust is module-scoped: if it's not pub, it's only
  accessible from within the same module.
- Modules can be defined within one file:

```rust
mod english {
    pub mod greetings {
    }
    pub mod farewells {
    }
}

mod japanese {
    pub mod greetings {
    }
    pub mod farewells {
    }
}
```

Reference: [TRPL 4.25](http://doc.rust-lang.org/book/crates-and-modules.html)

---
## Modules

```rust
mod english {
    pub mod greetings { /* ... */ }
}
```

- Modules can be defined as files instead:
- `lib.rs`:
    ```rust
    mod english;
    ```
- `english.rs`:
    ```rust
    pub mod greetings { /* ... */ }
    ```

---
## Modules

```rust
mod english {
    pub mod greetings { /* ... */ }
}
```

- Modules can also be defined as directories:
- `lib.rs`:
    ```rust
    mod english;
    ```
- `english/`
    - `mod.rs`:
        ```rust
        pub mod greetings;
        ```
    - `greetings.rs`:
        ```rust
        /* ... */
        ```

---
## Namespacing

- When accessing a member of a module, by default, namespaces
  are relative to the current module:

```rust
mod one {
    mod two { pub fn foo() {} }
    fn bar() {
        two::foo()
    }
}
```

- But it can be made absolute with a leading `::` operator:

```rust
mod one {
    mod two { pub fn foo() {} }
    fn bar() {
        ::one::two::foo()
    }
}
```

---
## `use`ing Modules

- `use` has the opposite rules.
- `use` directives are absolute by default:

```rust
use english::greetings;
```

- But can be relative to the current module:

```rust
// english/mod.rs
use self::greetings;
use super::japanese;
```

- `pub use` can be used to re-export other items:

```rust
// default_language.rs

#[cfg(english)]
pub use english::*;

#[cfg(japanese)]
pub use japanese::*;
```

---
## Using External Crates

- For external crates, use `extern crate` instead of `mod`.

```rust
extern crate rand;

use rand::Rng;
```

---
## Making Your Own Crate

- We've been writing lib crates - but how do we export from them?
- Anything marked `pub` in the root module (`lib.rs`) is exported:

```rust
pub mod english;
```

- Easy!

---
## Using Your Own Crate

- Now, you can use your own crate from Cargo:

```toml
[dependencies]
myfoo = { git = "https://github.com/me/foo-rs" }
mybar = { path = "../rust-bar" }
```

- Or:

```toml
[dependencies.myfoo]
git = "https://github.com/me/foo-rs"
```

- And use them:

```rust
extern crate myfoo;

use myfoo::english;
```

---
## Cargo: you got your bins in my lib

- We've seen both lib and bin (executable) crates in homework
    - Executable-only crates don't export any importable crates.
    - But this isn't _really_ a distinction!
- Cargo allows _both_ `:/src/lib.rs` and `:/src/main.rs`.
    - Cargo will also build `:/src/bin/*.rs` as executables.
- Examples go in `:/examples/*.rs`.
    - Built by `cargo test` (to ensure examples always build).
    - Can be called with `cargo run --example foo`.
- Integration (non-unit) tests go in `:/tests/*.rs`.
- Benchmarks go in `:/benches/*.rs`.

---
## Cargo: Features

- Features of a crate can be toggled at build time:
    - `cargo build --features using-html9`

```toml
[package]
name = "myfacebumblr"

[features]
# Enable default dependencies: require web-vortal *feature*
default = ["web-vortal"]

# Extra feature; now we can use #[cfg(feature = "web-vortal")]
web-vortal = []

# Also require h9rbs-js *crate* with its commodore64 feature.
using-html9 = ["h9rbs-js/commodore64"]

[dependencies]
# Optional dependency can be enabled by either:
# (a) feature dependencies or (b) extern crate h9rbs_js.
h9rbs-js = { optional = "true" }
```

---
## Cargo: Build Scripts

- Sometimes, you need more than what Cargo can provide.
- For this, we have build scripts!
    - Of course, they're written in Rust.

```toml
[package]
build = "build.rs"
```

- Now, `cargo build` will compile and run `:/build.rs` first.

---
## Cargo: The Rabbit Hole

- Cargo has a lot of features. If you're interested, check them out
  in the [Cargo manifest format][] documentation.

[Cargo manifest format]: http://doc.crates.io/manifest.html


---
# Attributes

- Ways to pass information to the compiler.
- `#[test]` is an attribute that annotates a function as a test.
- `#[test]` annotates the next block; `#![test]` annotates the surrounding block.

```rust
#[test]
fn midterm1() {
    // ...
}

fn midterm2() {
    #![test]
    // ...
}
```

---
## Attributes

- Use attributes to...
    - `#![no_std]` disable the standard library.
    - `#[derive(Debug)]` auto-derive traits.
    - `#[inline(always)]` give compiler behavior hints.
    - `#[allow(missing_docs)]` disable compiler warnings for certain lints.
    - `#![crate_type = "lib"]` provide crate metadata.
    - `#![feature(box_syntax)]` enable unstable syntax.
    - `#[cfg(target_os = "linux")]` define conditional compilation.
    - And [many more][reference/attributes]!

[reference/attributes]: https://doc.rust-lang.org/stable/reference.html#attributes

---
# Rust Code Style

---
## Rust Code Style

- A [style guide][] is being _drafted_ as part of the Rust docs.
- The main reason for many of the rules is to prevent pointless
  arguments about things like spaces and braces.
    - If you contribute to an open-source Rust project, it will probably be
      expected that you follow these rules.
- The [rustfmt][] project is an automatic code formatter.

[style guide]: https://github.com/rust-lang/rust/tree/master/src/doc/style
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt

---
## Spaces

- Lines must not exceed 99 characters.
- Use 4 spaces for indentation, not tabs.
- No trailing whitespace at the end of lines or files.
- Use spaces around binary operators: `x + y`.
- Put spaces after, but not before, commas and colons: `x: i32`.
- When line-wrapping function parameters, they should align.
    ```rust
    fn frobnicate(a: Bar, b: Bar,
                  c: Bar, d: Bar)
                  -> Bar {
    }
    ```

---
## Braces

- Opening braces always go on the same line.
- Match arms get braces, except for single-line expressions.
- `return` statements get semicolons.
- Trailing commas (in structs, matches, etc.) should be included if the
  closing delimiter is on a separate line.

---
## Capitalization & Naming

- You may have seen built-in lints on how to spell identifiers.
    - `CamelCase`: types, traits.
    - `lowerCamelCase`: not used.
    - `snake_case`: crates, modules, functions, methods, variables.
    - `SCREAMING_SNAKE_CASE`: static variables and constants.
    - `T` (single capital letter): type parameters.
    - `'a` (tick + short lowercase name): lifetime parameters.
- Constructors and conversions should be worded:
    - `new`, `new_with_stuff`: constructors.
    - `from_foo`: conversion constructors.
    - `as_foo`: free non-consuming conversion.
    - `to_foo`: expensive non-consuming conversion.
    - `into_foo`: consuming conversion.

---
## Advanced `format!`ing

- The `?` means debug-print. But what goes before the `:` part?
    - A _positional parameter_! An index into the argument list.

```rust
println!("{2} {} {} {0} {} {}", 0, 1, 2, 3) // ==> "2 0 1 0 2 3"
```

- Among the specifiers with no positional parameter, they implicitly
  count up: `{0} {1} {2} ...`.

- There are also _named parameters_:

```rust
format!("{name} {}", 1, name = 2); // ==> "2 1"
```

---
## `format!` Specifiers

- We've been printing stuff out with `println!("{:?}", bst);`
- There are more format specifiers than just `{}` and `{:?}`.
    - These all call traits in `std::fmt`:

| Spec.  | Trait    | Spec.  | Trait    | Spec.  | Trait    |
| ------ | -------- | ------ | -------- | ------ | -------- |
| `{}`   | Display  | `{:?}` | Debug    | `{:o}` | Octal    |
| `{:x}` | LowerHex | `{:X}` | UpperHex | `{:p}` | Pointer  |
| `{:b}` | Binary   | `{:e}` | LowerExp | `{:E}` | UpperExp |

---
## `format!` Specifiers

- There are tons of options for each of these format specifiers.
- Examples:
    - `{:04}` -> `0010`: padding
    - `'{:^4}'` -> `' 10 '`:  alignment (centering)
    - `#` indicates an "alternate" print format:
    - `{:#X}` -> `0xA`: including `0x`
    - `{:#?}`: Pretty-prints objects:

```
A {
    x: 5,
    b: B {
        y: 4
    }
}
```

- Complete reference: [std::fmt](https://doc.rust-lang.org/std/fmt/)

---
## Operators

- Operators are evaluated left-to-right, in the following order:
    - Unary operators: `!` `-` `*` `&` `&mut`
    - `as` casting
    - `*` `/` `%` multiplicative arithmetic
    - `+` `-` additive arithmetic
    - `<<` `>>` shift arithmetic
    - `&` bitwise and
    - `^` bitwise xor
    - `|` bitwise or
    - `==` `!=` `<` `>` `<=` `>=` logical comparison
    - `&&` logical and
    - `||` logical or
    - `=` `..` assignment and ranges
- Also: `call()`, `index[]`

---
## Operator Overloading

- Okay, same old, same old. We can customize these!
- Rust defines these - surprise! - using traits, in `std::ops`.
    - `Neg`, `Not`, `Deref`, `DerefMut`
    - `Mul`, `Div`, `Mod`
    - `Add`, `Sub`
    - `Shl`, `Shr`
    - `BitAnd`
    - `BitXor`
    - `BitOr`
    - `Eq`, `PartialEq`, `Ord`, `PartialOrd`
    - `And`
    - `Or`
- Also: `Fn`, `FnMut`, `FnOnce`, `Index`, `IndexMut`, `Drop`

---
### `From` One Type `Into` Another

- Casting (`as`) cannot be overloaded - instead, we use `From` and `Into`.
    - `trait From<T> { fn from(T) -> Self; }`, called like `Y::from(x)`.
    - `trait Into<T> { fn into(self) -> T; }`, called like `x.into()`.
- If you implement `From`, `Into` will be automatically implemented.
    - So you should prefer implementing `From`.

```rust
struct A(Vec<i32>);
impl From<Vec<i32>> for A {
    fn from(v: Vec<i32>) -> Self {
        A(v)
    }
}
```

---
### `From` One Type `Into` Another

- But sometimes, for various reasons, implementing `From` isn't possible - only `Into`.

```rust
struct A(Vec<i32>);

impl From<A> for Vec<i32> { // error: private type A in
    fn from(a: A) -> Self { // exported type signature.
        let A(v) = a; v     // (This impl is exported because
    }                       // both the trait (From) and the type
}                           // (Vec) are visible from outside.)

impl Into<Vec<i32>> for A {
    fn into(self) -> Vec<i32> {
        let A(v) = self; v
    }
}
```

---
### Making References

- `Borrow`/`BorrowMut`: "a trait for borrowing data."&sup1;

```rust
trait Borrow<Borrowed> { fn borrow(&self) -> &Borrowed; }
```

- `AsRef`/`AsMut`: "a cheap, reference-to-reference conversion."&sup2;

```rust
trait AsRef<T>         { fn as_ref(&self) -> &T; }
```

- So... they're exactly the same?

&sup1; [Trait std::borrow::Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) &sup2; [Trait std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

- Citing the docs: `AsRef` is to be used when wishing to convert to a reference of another type. `Borrow` is more related to the notion of taking the reference. It is useful when wishing to abstract over the type of reference (`&T`, `&mut T`) or allow both the referenced and owned type to be treated in the same manner.

