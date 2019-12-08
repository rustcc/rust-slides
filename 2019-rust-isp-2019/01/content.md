# Generics & Traits

### Rust ISP 2019 Lecture 01
Based on: [CIS 198 slides](https://github.com/cis198-2016s/slides)

Artyom Pavlov, 2019.

---
## Generics

- Suppose we simplify the `Resultish` enum from the last lecture a bit...

```rust
enum Result {
    Ok(String),
    Err(String),
}
```
- Better, but it's still limited to passing two values which are both `String`s.

---
## Generics

- This looks a lot like a standard library enum, `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- `T` and `E` stand in for any generic type, not only `String`s.
- You can use any CamelCase identifier for generic types.

---
## Generic Structs

- Let's take a look at generic versions of several other structs from the last lecture:

```rust
struct Point<T> {
    x: T,
    y: T,
}

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
```

---
## Generic Implementations

- To define implementations for structs & enums with generic types, declare the generics at the
    beginning of the `impl` block:

```rust
impl<T, E> Result<T, E> {
    fn is_ok(&self) -> bool {
        match *self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }
}
```

- Or with "[match ergonomics](https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md)":

```rust
match self {
    Result::Ok(_) => true,
    Result::Err(_) => false,
}
```

---
## Traits

- Implementing functions on a per-type basis to pretty-print, compute equality, etc. is
    fine, but unstructured.
- We currently have no abstract way to reason about what types can do what!

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn format(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    fn equals(&self, other: Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

---
## Traits

- Solution: Traits (coming right now)!
- These are similar to Java/Go interfaces or Haskell typeclasses

---
## Traits

- To define a trait, use a `trait` block, which gives function definitions for
  the required methods.
    - This is not the same as an `impl` block.
    - Mostly only contains method signatures without definitions.

```rust
trait PrettyPrint {
    fn format(&self) -> String;
}
```

---
## Traits

- To implement a trait, use an `impl Trait for Type` block.
    - All methods specified by the trait must be implemented.
- One impl block per type per trait.
- You can use `self`/`&self` inside the trait `impl` block as usual.

```rust
struct Point {
    x: i32,
    y: i32,
}

impl PrettyPrint for Point {
    fn format(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
```

---
## Generic Functions

- You can make a function generic over types as well.
- `<T, U>` declares the type parameters for `foo`.
    - `x: T, y: U` uses those type parameters.
- You can read this as "the function `foo`, for all types `T` and `U`,
    of two arguments: `x` of type `T` and `y` of type `U`."

```rust
fn foo<T, U>(x: T, y: U) {
    // ...
}
```

- But we can't do much with `x` and `y`, because we don't know anything
    about types `T` and `U`

---
## Generics with Trait Bounds

- Instead of allowing _literally any_ type, you can constrain generic types by
    _trait bounds_.
- This gives more power to generic functions & types.
- Trait bounds can be specified with `T: SomeTrait`, with a `where` clause,
    or with `impl Trait` in argument position.

```rust
fn cloning_machine<T: Clone>(t: T) -> (T, T) {
    (t.clone(), t.clone())
}

fn cloning_machine_2<T>(t: T) -> (T, T)
        where T: Clone {
    (t.clone(), t.clone())
}

fn cloning_machine_3(t: impl Clone) -> (T, T) {
    (t.clone(), t.clone())
}
```

---
## Generics with Trait Bounds

- Multiple trait bounds are specified like `T: Clone + Ord`.
- There's no way (yet) to specify [negative trait bounds](https://internals.rust-lang.org/t/pre-rfc-mutually-exclusive-traits/2126).
  - e.g. you can't stipulate that a `T` must not be `Clone`.

```rust
fn clone_and_compare<T: Clone + Ord>(t1: T, t2: T) -> bool {
   t1.clone() > t2.clone()
}
```

---
## Generic Types With Trait Bounds

- You can also define structs with generic types and trait bounds.
- Be sure to declare all of your generic types in the struct header _and_ the
  impl block header.
- Only the impl block header needs to specify trait bounds.
    - This is useful if you want to have multiple impls for a struct each with
      different trait bounds

```rust
struct Point<T: Add> {
    x: T,
    y: T,
}
```

---
## Generic Types With Trait Bounds

```rust
trait PrettyPrint {
   fn format(&self) -> String;
}

impl<T: PrettyPrint, E: PrettyPrint> PrettyPrint for Result<T, E> {
   fn format(&self) -> String {
      match self {
         Ok(t) => format!("Ok({})", t.format()),
         Err(e) => format!("Err({})", e.format()),
      }
   }
}
```

---
## Examples: Equality

```rust
// This is not the trait Rust actually uses for equality
trait Equals {
    fn equals(&self, other: &Self) -> bool;
}

impl<T: Equals, E: Equals> Equals for Result<T, E> {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(t1), Ok(t2)) => t1.equals(t2),
            (Err(e1), Err(e2)) => e1.equals(e2),
            _ => false
        }
    }
}
```
- `Self` is a special type which refers to the type of `self`.

---
## Inheritance (kinda)

- Some traits may require other traits to be implemented first.
    - e.g., `Eq` requires that `PartialEq` be implemented, and `Copy` requires `Clone`.
- Implementing the `Child` trait below requires you to also implement `Parent`.

```rust
trait Parent {
    fn foo(&self) {
        // ...
    }
}

trait Child: Parent {
    fn bar(&self) {
        self.foo();
        // ...
    }
}
```

---
## Default Methods

- Traits can have default implementations for methods!
   - Useful if you have an idea of how an implementor will commonly define a trait method.
- When a default implementation is provided, the implementor of the trait doesn't need to define that method.
- Define default implementations of trait methods by simply writing the body in
    the `trait` block.

```rust
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

trait Eq: PartialEq<Self> {}
```

---
## Default Methods

- Implementors of the trait can overwrite default implementations, but make sure
    you have a good reason to!
    - e.g., _never_ define `ne` so that it violates the relationship between
      `eq` and `ne`.

---
## Deriving

- Many traits are so straightforward that the compiler can often implement them
  for you.
- A `#[derive(...)]` attribute tells the compiler to insert a default
  implementation for whatever traits you tell it to.
- This removes the tedium of repeatedly manually implementing traits like `Clone` yourself!

```rust
#[derive(Eq, PartialEq, Debug)]
enum Result<T, E> {
   Ok(T),
   Err(E)
}
```

---
## Deriving

- You can derive the following core traits:
    - `Clone`, `Copy`, `Debug`, `Default`, `Eq`,
    - `Hash`, `Ord`, `PartialEq`, `PartialOrd`.
- Deriving custom traits is also supported! (e.g. see [`serde`](https://serde.rs/) and [`diesel`](http://diesel.rs/))
- Careful: deriving a trait won't always work.
    - Can only derive a trait on a data type when all of its members can have derived the trait.
    - e.g., `Eq` can't be derived on a struct containing only `f32`s, since
      `f32` is not `Eq`.

---
## Core traits

- It's good to be familiar with the core traits.
    - `Clone`, `Copy`
    - `Debug`
    - `Default`
    - `Eq`, `PartialEq`
    - `Hash`
    - `Ord`, `PartialOrd`

---
### Clone

```rust
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}
```
- A trait which defines how to duplicate a value of type `T`.
- This can solve ownership problems.
    - You can clone an object rather than taking ownership or borrowing!
- [Documentation](https://doc.rust-lang.org/std/clone/trait.Clone.html)

---
### Clone

```rust
#[derive(Clone)] // without this, Bar cannot derive Clone.
struct Foo {
    x: i32,
}

#[derive(Clone)]
struct Bar {
    x: Foo,
}
```

---
### Copy
```rust
pub trait Copy: Clone { }
```
- `Copy` denotes that a type has "copy semantics" instead of "move semantics."
- Type must be able to be copied by copying bits (`memcpy`).
    - Types that contain references _cannot_ be `Copy`.
- Marker trait: does not implement any methods, but defines behavior instead.
- [Documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)

---
### Debug

```rust
pub trait Debug {
    fn fmt(&self, &mut Formatter) -> Result;
}
```

- Defines output for the `{:?}` formatting option.
- Generates debug output, not pretty printed.
- Generally speaking, you should always derive this trait.

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };
println!("The origin is: {:?}", origin);
// The origin is: Point { x: 0, y: 0 }
```
- [Documentation](https://doc.rust-lang.org/std/fmt/trait.Debug.html)

---
### Default

```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```
- Defines a default value for a type.
- Often can be derived, but sometimes it has to be implemented explicitly.
- [Documentation](https://doc.rust-lang.org/std/default/trait.Default.html)

---
### Eq vs. PartialEq

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool { ... }
}

pub trait Eq: PartialEq<Self> {}
```
- Traits for defining equality via the `==` operator.

---
### Eq vs. PartialEq

- `PartialEq` represents a _partial equivalence relation_.
    - Symmetric: if a == b then b == a
    - Transitive: if a == b and b == c then a == c
- `ne` has a default implementation in terms of `eq`.
- `Eq` represents a _total equivalence relation_.
    - Symmetric: if a == b then b == a
    - Transitive: if a == b and b == c then a == c
    - **Reflexive: a == a**
- `Eq` does not define any additional methods.
    - (It is also a Marker trait.)
- For example, in floating point numbers `NaN != NaN`,
    so floating point types implement `PartialEq` but not `Eq`.

---
### Ord vs. PartialOrd

```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    // Ordering is one of Less, Equal, Greater
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
```
- Traits for values that can be compared for a sort-order.

---
### Ord vs. PartialOrd

- The comparison must satisfy, for all `a`, `b` and `c`:
  - Antisymmetry: if `a < b` then `!(a > b)`, as well as `a > b` implying `!(a < b)`; and
  - Transitivity: `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
- `lt`, `le`, `gt`, `ge` have default implementations based on `partial_cmp`.

---
### Ord vs. PartialOrd

```rust
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
}
```
- Trait for types that form a total order.
- An order is a total order if it is (for all `a`, `b` and `c`):
  - total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true; and
  - transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
- For example, for floating point numbers, `NaN < 0 == false` and `NaN >= 0 == false` (cf. IEEE 754-2008 section 5.11).
- See `std::cmp` [documentation](https://doc.rust-lang.org/std/cmp/index.html) for more information.

---
### Hash

```rust
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
        where Self: Sized { ... }
}
```
- A hashable type.
- The `H` type parameter is an abstract hash state used to compute the hash.
- If you also implement `Eq`, there is an additional, important property:
```rust
k1 == k2 -> hash(k1) == hash(k2)
```
- [Documentation](https://doc.rust-lang.org/std/hash/trait.Hash.html)

---
## Associated Types

- Take this `Graph` trait from the Rust book:

```rust
trait Graph<N, E> {
    fn edges(&self, &N) -> Vec<E>;
    // etc
}
```

- `N` and `E` are generic type parameters, but they don't have any meaningful
    association to `Graph`
- Also, any function that takes a `Graph` must also be generic over `N` and `E`!

```rust
fn distance<N, E, G: Graph<N,E>>(graph: &G, start: &N, end: &N)
    -> u32 { /*...*/ }
```

---
## Associated Types

- Solution: associated types!
- `type` definitions inside a trait block indicate associated generic types on
    the trait.
- An implementor of the trait may specify what the associated types correspond
    to.

```rust
trait Graph {
  type N;
  type E;

  fn edges(&self, &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
  type N = MyNode;
  type E = MyEdge;

  fn edges(&self, n: &MyNode) -> Vec<MyEdge> { /*...*/ }
}
```

---
## Associated Types

- For example, in the standard library, traits like `Iterator` define an `Item` associated type.
- Methods on the trait like `Iterator::next` then return an `Option<Self::Item>`!
    - This lets you easily specify what type a client gets by iterating over
        your collection.

---
## Trait Scope

- Say our program defines some trait `Foo`.
- It's possible to implement this trait on any type in Rust, including types that
  you don't own:

```rust
trait Foo {
   fn bar(&self) -> bool;
}

impl Foo for i32 {
    fn bar(&self) -> bool {
        true
    }
}
```

---
## Trait Scope

- The scope rules for implementing traits:
    - You need to `use` a trait in order to access its methods on types, even if
      you have access to the type.
    - In order to write an `impl`, you need to own (i.e. have yourself defined)
      either the trait or the type.

---
### Display

```rust
pub trait Display {
    fn fmt(&self, &mut Formatter) -> Result<(), Error>;
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {}, {})", self.x, self.y)
    }
}
```

- Defines output for the `{}` formatting option.
- Like Debug, but should be pretty printed.
    - No standard output and cannot be derived!
- You can use `write!` macro to implement this without using Formatter.

---
## Addendum: Drop

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

- A trait for types that are destructable (which is all types).
- `Drop` requires one method, `drop`, but you should never call this method yourself.
    - It's inserted automatically by the compiler when necessary.

---
## Addendum: Drop

- Typically, you won't actually implement `Drop` for a type
    - Generally the default implementation is fine.
    - You also don't need to `derive` `Drop` either.
- Why implement `Drop` then?
    - If you need some special behavior when an object gets destructed.

---
## Addendum: Drop

- Example: Rust's reference-counted pointer type `Rc<T>` has special `Drop` rules:
    - If the number of references to an `Rc` pointer is greater than 1, `drop` decrements the ref count.
    - The `Rc` is actually deleted when the reference count drops to 0.

---
## Addendum: `Sized` vs. `?Sized`

- `Sized` indicates that a type has a constant size known at compile time!
- Its evil twin, `?Sized`, indicates that a type _might_ be sized.
- By default, all types are implicitly `Sized`, and `?Sized` undoes this.
    - Types like `[T]` and `str` (no `&`) are `?Sized`.
- For example, `Box<T>` allows `T: ?Sized`.
- You rarely interact with these traits directly, but they show up a lot in trait bounds.

---
## Trait Objects

- Consider the following trait, and its implementors:

```rust
trait Foo { fn bar(&self); }

impl Foo for String {
    fn bar(&self) { /*...*/ }
}

impl Foo for usize {
    fn bar(&self) { /*...*/  }
}
```

---
## Trait Objects

- We can call either of these versions of `bar` via static dispatch using any type with bounds `T: Foo`.
- When this code is compiled, the compiler will insert calls to specialized versions of `bar` (monomorphization)
    - One function is generated for each implementor of the `Foo` trait.

```rust
fn blah(x: T) where T: Foo {
    x.bar()
}

fn main() {
    let s = "Foo".to_string();
    let u = 12;

    blah(s);
    blah(u);
}
```

---
## Trait Objects

- It is also possible to have Rust perform _dynamic_ dispatch through the use of *trait objects*.
- A trait object is something like `Box<Foo>` or `&Foo`
- The data behind the reference/box must implement the trait `Foo`.
- The concrete type underlying the trait is erased; it can't be determined.
- Well, there is ways to downcast a trait object to a concrete type if you really need to,
    but usually you don't.

---
## Trait Objects

```rust
trait Foo { /*...*/ }

impl Foo for char { /*...*/ }
impl Foo for i32  { /*...*/ }

fn use_foo(f: &Foo) {
    // No way to figure out if we got a `char` or an `i32`
    // or anything else!
    match *f {
        // What type do we have? I dunno...
        // error: mismatched types: expected `Foo`, found `_`
        198 => println!("CIS 198!"),
        'c' => println!("See?"),
        _ => println!("Something else..."),
    }
}

use_foo(&'c'); // These coerce into `&Foo`s
use_foo(&198i32);
```

---
## Trait Objects

- When a trait object is used, method dispatch must be performed at runtime.
    - The compiler can't know the type underlying the trait reference, since it was erased.
- This causes a runtime penalty, but is useful when handling things like dynamically sized types.

---
## Object Safety

- Not all traits can be safely used in trait objects!
- Trying to create a variable of type `&Clone` will cause a compiler error, as `Clone` is not _object safe_.
- A trait is object-safe if:
    - It does not require that `Self: Sized`
    - Its methods must not use `Self`
    - Its methods must not have any type parameters
    - Its methods do not require that `Self: Sized`

---
### Addendum: Generics With Lifetime Bounds

- Some generics may have lifetime bounds like `T: 'a`.
- Semantically, this reads as "Type `T` must live at least as long as the lifetime `'a`."
- Why is this useful?
- Imagine you have some collection of type `T`.
- If you iterate over this collection, you should be able to guarantee that
    everything in it lives as long as the collection.
    - If you couldn't, Rust wouldn't be safe!
- `std::Iterator` structs usually contain these sorts of constraints.

---
## Exercise

- Solve "Traits" exercise from: http://www.rust-tutorials.com/exercises/

---
## Closures
- A closure, anonymous function, or lambda function is a common paradigm in
  functional languages.
- In Rust, they're fairly robust, and match up well with the rest of Rust's
  ownership model.

```rust
let square = |x: i32| -> i32 { x * x };
println!("{}", square(3));
// => 6
```

---
## Closure Syntax

```rust
let foo_v1 = |x: i32| { x * x };
let foo_v2 = |x: i32, y: i32| x * y;
let foo_v3 = |x: i32| {
    // Very Important Arithmetic
    let y = x * 2;
    let z = 4 + y;
    x + y + z
};
let foo_v4 = |x: i32| if x == 0 { 0 } else { 1 };
```

- These look pretty similar to function definitions.
- Specify arguments in `||`, followed by the return expression.
    - The return expression can be a series of expressions in `{}`.

---
## Type Inference

```rust
let square_v4 = |x: u32| { (x * x) as i32 };

let square_v4 = |x| -> i32 { x * x }; // ← unable to infer enough
let square_v4 = |x|        { x * x }; // ← type information!
```

- Unlike functions, we don't _need_ to specify the return type or argument types
  of a closure.
    - In this case, the compiler can't infer the type of the argument `x` from
      the return expression `x * x`.

---
## Closure Environment

- Closures _close_ over (contain) their environment.

```rust
let magic_num = 5;
let magic_johnson = 32;
let plus_magic = |x: i32| x + magic_num;
```

- The closure `plus_magic` is able to reference `magic_num` even though it's not
  passed as an argument.
    - `magic_num` is in the "environment" of the closure.
    - `magic_johnson` is not borrowed!

---
## Closure Environment

- If we try to borrow `magic_num` in a conflicting way after the
  closure is bound, we'll get an error from the compiler:

```rust
let mut magic_num = 5;
let magic_johnson = 32;
let plus_magic = |x: i32| x + magic_num;

let more_magic = &mut magic_num; // Err!
println!("{}", magic_johnson); // Ok!
```

```
  error: cannot borrow `magic_num` as mutable because it is
  already borrowed as immutable

  [...] the immutable borrow prevents subsequent moves or mutable
  borrows of `magic_num` until the borrow ends
```

- Why? `plus_magic` borrows `magic_num` when it closes over it!
- However, `magic_johnson` is not used in the closure, and its ownership is not
  affected.

---
## Closure Environment

- We can fix this kind of problem by making the closure go out of scope:

```rust
let mut magic_num = 5;
{
    let plus_magic = |x: i32| x + magic_num;
} // the borrow of magic_num ends here

let more_magic = &mut magic_num; // Ok!
println!("magic_num: {}", more_magic);
```

---
## Move Closures

- As usual, closures are choose-your-own-~~adventure~~ ownership.
- Sometimes it's not okay to have a closure borrow _anything_.
- You can force a closure to _take ownership_ of all environment
  variables by using the `move` keyword.
    - "Taking ownership" can mean taking a copy, not just moving.

```rust
let mut magic_num = 5;
let own_the_magic = move |x: i32| x + magic_num;
let more_magic = &mut magic_num;
```

---
## Move Closures

- `move` closures are necessary when the closure `f` needs to outlive the scope in
  which it was created.
    - e.g. when you pass `f` into a thread, or return `f` from a function.
    - `move` essentially _disallows_ bringing references into the closure.

```rust
fn make_closure(x: i32) -> Box<Fn(i32) -> i32> {
    let f = move |y| x + y; // ^ more on this in 15 seconds
    Box::new(f)
}

let f = make_closure(2);
println!("{}", f(3));
```

---
## Closure Ownership

- Sometimes, a closure _must_ take ownership of an environment variable to be
  valid. This happens automatically (without `move`):

    - If the value is moved into the return value.
        ```rust
        let lottery_numbers = vec![11, 39, 51, 57, 75];
        {
            let ticket = || { lottery_numbers };
        }
        // The braces do no good here.
        println!("{:?}", lottery_numbers); // use of moved value
        ```

    - Or moved anywhere else.
        ```rust
        let numbers = vec![2, 5, 32768];
        let alphabet_soup = || { numbers; vec!['a', 'b'] };
                              // ^ throw away unneeded ingredients
        println!("{:?}", numbers); // use of moved value
        ```

- If the type is not `Copy`, the original variable is invalidated.

---
## Closure Ownership

```rust
let numbers = vec![2, 5, 32768];
let alphabet_soup = || { numbers; vec!['a', 'b'] };
                      // ^ throw away unneeded ingredients
alphabet_soup();
alphabet_soup(); // use of moved value
```

- Closures which own data and then move it can only be called once.
    - `move` behavior is implicit because `alphabet_soup` must own `numbers` to
      move it.

```rust
let numbers = vec![2, 5, 32768];
let alphabet_soup = move || { println!("{:?}", numbers) };
alphabet_soup();
alphabet_soup(); // Delicious soup
```

- Closures which own data but don't move it can be called multiple times.

---
## Closure Ownership

- The same closure can take some values by reference and others by moving
  ownership (or Copying values), determined by behavior.

---
## Closure Traits

- Closures are actually based on a set of traits under the hood!
    - `Fn`, `FnMut`, `FnOnce` - method calls are overloadable operators.

```rust
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call"
      fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call"
      fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call"
      fn call_once(self, args: Args) -> Self::Output;
}
```

---
## Closure Traits

- These traits all look pretty similar, but differ in the way they take `self`:
    - `Fn` borrows `self` as `&self`
    - `FnMut` borrows `self` mutably as `&mut self`
    - `FnOnce` takes ownership of `self`
- `Fn` is a superset of `FnMut`, which is a superset of `FnOnce`.
- Functions also implement these traits.

- The `|| {}` syntax for closures is sugar for these three traits. Rust will
generate a struct for the environment, impl the appropriate trait, and then use
it.

---
## Closures As Arguments

- Passing closures works like function pointers.
- Let's take a (simplified) look at Rust's definition for `map`.

```rust
// self = Vec<A>
fn map<A, B, F>(self, f: F) -> Vec<B>
    where F: FnMut(A) -> B;
```

- `map` takes an argument `f: F`, where `F` is an `FnMut` trait object.
- You can pass regular functions in, since the traits line up!

---
## Returning Closures

- You may find it necessary to return a closure from a function.
- Unfortunately, since closures are implicitly trait objects, they're unsized!

```rust
fn i_need_some_closure() -> (Fn(i32) -> i32) {
    let local = 2;
    |x| x * local
}
```

```
error: the trait `core::marker::Sized` is not implemented
    for the type `core::ops::Fn(i32) -> i32 + 'static`
```

- An `Fn` object is not of constant size at compile time.
    - The compiler cannot properly reason about how much space to allocate for the `Fn`.

---
## Returning Closures

- Okay, we can fix this! Just wrap the `Fn` in a layer of indirection and return a reference!

```rust
fn i_need_some_closure_by_reference() -> &(Fn(i32) -> i32) {
    let local = 2;
    |x| x * local
}
```

```
error: missing lifetime specifier
```

- Now what? We haven't given this closure a lifetime specifier...
    - The reference we're returning must outlive this function.
    - But it can't, since that would create a dangling pointer.

---
## Returning Closures

- What's the right way to fix this? Use a `Box`!

```rust
fn box_me_up_that_closure() -> Box<Fn(i32) -> i32> {
    let local = 2;
    Box::new(|x| x * local)
}
```

```
error: closure may outlive the current function, but it
borrows `local`, which is owned by the current function [E0373]
```

- Augh! We were so close!
- The closure we're returning is still holding on to its environment.
    - That's bad, since once `box_me_up_that_closure` returns, `local` will be destroyed.

---
## Returning Closures
- The good news? We already know how to fix this:

```rust
fn box_up_your_closure_and_move_out() -> Box<Fn(i32) -> i32> {
    let local = 2;
    Box::new(move |x| x * local)
}
```

- And you're done. It's elementary!

---
## String Types

- Rust strings are complicated.
    - Sequences of Unicode values encoded in UTF-8.
    - Not null-terminated and may contain null bytes.
- There are two kinds: `&str` and `String`.

---
## `&str`

- `&str` is a string slice (like array slice).
- `"string literals"` are of type `&str`.&sup1;
- `&str`s are statically-allocated and fixed-size. (pointer + length under the hood)
- May not be indexed with `some_str[i]`, as each character
    may be multiple bytes due to UTF-8 encoding.
- Instead, iterate with `chars()`:
    - `for c in "1234".chars() { ... }`
- As with all Rust references, they have an associated lifetime.

&sup1;More specifically, they have the type `&'static str`.

---
## `String`

- `String`s are heap-allocated, and are dynamically growable.
    - Like `Vec`s in that regard.
    - In fact, `String` is just a wrapper over `Vec<u8>`!
- Cannot be indexed either.
    - You can select characters with `s.chars().nth(i)`.
- May be coerced into an `&str` by taking a reference to the `String`.

```rust
let s0: String = String::new();
let s1: String = "foo".to_string();
let s2: String = String::from("bar");
let and_s: &str = &s0;
```

---
## `str`

- If `&str` is the second string type, what exactly is `str`?
- An `Unsized` type, meaning the size is unknown at compile time.
    - You can't have bindings to `str`s directly, only references.

---
## String Concatenation

- A `String` and an `&str` may be concatenated with `+`:

```rust
let course1 = "Rust".to_string();
let course2 = course1 + " ISP";
```

- Concatenating two `String`s requires coercing one to `&str`:

```rust
let course1 = String::from("Rust");
let course2  = String::from(" ISP");
let course3 = course1 + &course2;
```

- You can't concatenate two `&str`s.

```rust
let course_name = "Rust " + "ISP"; // Err!
```

---
## String Conversion

- `String` can be converted to `&str` via "deref coercions".
- `std::ops::Deref` trait allows automatic conversions of references,
    in our case it converts `&String` to `&str`.

```rust
use std::net::TcpStream;

TcpStream::connect("192.168.0.1:3000"); // &str
let addr = "192.168.0.1:3000".to_string();
TcpStream::connect(&addr);
```

- This doesn't automatically coerce because `TcpStream` doesn't take an argument
  of type `&str`, but a Trait bounded type:
    - `TcpStream::connect<A: ToSocketAddr>(addr: A);`

---
### `Deref` Coercions

- Rust's automatic dereferencing behavior works *between* types as well.

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```
- Since `String` implements `Deref<Target=str>`, so values of `&String` will
  automatically be dereferenced to `&str` when possible.

---
## `String` & `&str`: Why?

- Like slices for `Vec`s, `&str`s are useful for passing a view into a `String`.
- It's expensive to copy a `String` around, and lending an entire `String` out
    may be overkill.
- `&str` therefore allows you to pass portions of a `String` around, saving
    memory.
- Generally, if you want to do more than use string literals, use `String`.
    - You can then lend out `&str`s easily.

```rust
let s1: &str = "Hello world!";
let s2: String = s1.to_string();
let s3: &str = &s2[3..8]; // "lo wo"
```

---
## `Option<T>`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- Provides a concrete type to the concept of _nothingness_.
- Use this instead of returning `NaN`, `-1`, `null`, etc. from a function.
- No restrictions on what `T` may be.

---
### `Option::unwrap()`

- The pattern where None values are ignored is pretty common:

```rust
// fn foo() -> Option<i32>

match foo() {
    None => None,
    Some(value) => {
        bar(value)
        // ...
    },
}
```

---
### `Option::unwrap()`

- What if we extracted the pattern match into a separate function to simplify it?

```rust
fn unwrap<T>(&self) -> T { // 🎁!
    match *self {
        None => panic!("Called `Option::unwrap()` on a `None` value"),
        Some(value) => value,
    }
}

let x = foo().unwrap();
let y = bar(x);
// ...
```

- Unfortunately, `panic!`ing on `None` values makes this abstraction inflexible.
- Better: use `expect(&self, msg: String) -> T` instead.
    - `panic!`s with a custom error message if a `None` value is found.

---
### `Option::map()`

- Let's make the pattern a little better.
- We'll take an `Option`, change the value if it exists, and return an `Option`.
    - Instead of failing on `None`, we'll keep it as `None`.

```rust
fn map<U, F>(self, f: F) -> Option<U>
        where F: FnOnce(T) -> U {
    match self {
        None => None,
        Some(x) => Some(f(x))
    }
}

// fn foo() -> Option<i32>

let x = foo().map(|x| bar(x));
```

---
### `Option::and_then()`

- There's a similar function `and_then`:

```rust
fn and_then<U, F>(self, f: F) -> Option<U>
      where F: FnOnce(T) -> Option<U> {
    match self {
        Some(x) => f(x),
        None => None,
    }
}

// fn foo() -> Option<i32>

let x = foo().and_then(|x| Some(bar(x)));
```

- Notice the type of `f` changes from `T -> U` to `T -> Some(U)`.

---
### `Option::unwrap_or()`

- If we don't want to operate on an `Option` value, but it has a sensible
  default value, there's `unwrap_or`.

```rust
impl<T> Option<T> {
    fn unwrap_or<T>(&self, default: T) -> T {
      match *self {
          None => default,
          Some(value) => value,
      }
    }
}
```

- Important! Default value passed to `unwrap_or` always gets evaluated.

---
### `Option::unwrap_or_else()`

- If you don't have a static default value, but you can write a closure to
  compute one:

```rust
impl<T> Option<T> {
    fn unwrap_or_else<T>(&self, f: F) -> T
            where F: FnOnce() -> T {
        match *self {
            None => f(),
            Some(value) => value,
        }
    }
}
```

---
### Other

- Some other methods provided by Option:
- `fn is_some(&self) -> bool`
- `fn is_none(&self) -> bool`
- `fn map_or<U, F>(self, default: U, f: F) -> U`
    - `where F: FnOnce(T) -> U`
    - A default value`: U`.
- `fn map_or_else<U, D, F>(self, default: D, f: F) -> U`
    - `where D: FnOnce() -> U, F: FnOnce(T) -> U`
    - A default-generating closure`: D`.

---
### Other

- `fn ok_or(self, err: E) -> Result<T, E>`
- `fn ok_or_else(self, default: F) -> Result<T, E>`
    - `where F: FnOnce() -> E`
    - Similar to `unwrap_or` but returns a `Result` with a default `Err` or closure.
- `fn and<U>(self, optb: Option<U>) -> Option<U>`
    - Returns `None` if `self` is `None`, else `optb`
- `fn or(self, optb: Option<T>) -> Option<T>`
    - returns `self` if `self` is `Some(_)`, else `optb`

---
## Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

- `Result` is like `Option`, but it also encodes an `Err` type.
- Also defines `unwrap()` and `expect()` methods.
- Can be converted to an `Option` using `ok()` or `err()`.
    - Takes either `Ok` or `Err` and discards the other as `None`.
- Can be operated on in almost all the same ways as `Option`
    - `and`, `or`, `unwrap`, etc.

---
## Result<T, E>

- Unlike `Option`, a `Result` should _always_ be consumed.
    - If a function returns a `Result`, you should be sure to `unwrap`/`expect`
        it, or otherwise handle the `Ok`/`Err` in a meaningful way.
    - The compiler warns you if you don't.
    - Not using a result could result (ha) in your program unintentionally
        crashing!

---
### Custom Result Aliases

- A common pattern is to define a type alias for Result which uses your libary's
  custom Error type.

```rust
use std::io::Error;

type Result<T> = Result<T, Error>;
```

- Typically a convenience alias; other than fixing `E = Error`, this is
  identical to `std::Result`.
- Users of this type should namespace it:

```rust
use std::io;

fn foo() -> io::Result {
    // ...
}
```

---
## Result - `try!`

- `try!` is a macro, which means it generates Rust's code at compile-time.
    - This means it can actually expand to pattern matching syntax patterns.
- The code that `try!` generates looks roughly like this:

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(err),
    });
}
```

---
## `?`

- `?` is a postfix operator for "bubbling" errors:

```rust
let socket1: TcpStream = TcpStream::connect("127.0.0.1:8000")?;

// Is equivalent to...
let maybe_socket: Result<TcpStream> =
    TcpStream::connect("127.0.0.1:8000");
let socket2: TcpStream =
    match maybe_socket {
        Ok(val) => val,
        Err(err) => { return Err(err) }
    };
```

- Works via (unstable) `Try` trait and can be used with `Option`.
- Can convert error types using `From` trait.
- Replaced (no deprcated) `try!` macro.

---
## [Collections](https://doc.rust-lang.org/stable/std/collections/)

<img src="img/collector.jpg" style="width: 400px;"/>

---
## `Vec<T>`

- Nothing new here.

---
## `VecDeque<T>`

- An efficient double-ended `Vec`.
- Implemented as a ring buffer.
- The "default" usage of this type as a queue is to use `push_back` to add to the queue, and `pop_front` to remove from the queue.
- `extend` and `append` push onto the back in this manner, and iterating over VecDeque goes front to back.

---
## `LinkedList<T>`

- A doubly-linked list.
- Even if you want this, you probably don't want this.
    - Seriously, did you even read *any* of Gankro's book?
- Check out "[Learning Rust With Entirely Too Many Linked Lists](https://cglab.ca/~abeinges/blah/too-many-lists/book/)"

---
## `HashMap<K,V>`/`BTreeMap<K,V>`

- Map/dictionary types.
- `HashMap<K, V>` is useful when you want a basic map.
    - Requires that `K: Hash + Eq`.
    - Uses "linear probing with Robin Hood bucket stealing".
- `BTreeMap<K, V>` is a sorted map (with slightly worse performance).
    - Requires that `K: Ord`.
    - Uses a B-tree under the hood (surprise surprise).

---
## `HashSet<T>`/`BTreeSet<T>`

- Sets for storing unique values.
- `HashSet<T>` and `BTreeSet<T>` are literally struct wrappers for `HashMap<T, ()>` and `BTreeMap<T, ()>`.
- Same tradeoffs and requirements as their Map variants.

---
## `BinaryHeap<T>`

- A priority queue implemented with a binary max-heap.

---
## Aside: semi-official crates

- Useful "stdlib-ish" crates that are community-developed, but not
    official-official.
- Crates like:
    - Bindings to `libc`: https://github.com/rust-lang/libc
    - A `rand` library: https://github.com/rust-random/rand
    - Regex support: https://github.com/rust-lang/regex
    - Serialization: https://github.com/serde-rs/serde
    - UUID generation: https://github.com/uuid-rs/uuid

---
## Iterators

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // More methods omitted
}
```

- A Trait with an associated type, `Item`, and a method `next` which yields that
  type.
- Other methods (consumers and adapters) are implemented on `Iterator` as
  default methods using `next`.

---
## Iterators

- Like everything else, there are three types of iteration:
    - `into_iter()`, yielding `T`s.
    - `iter()`, yielding `&T`s.
    - `iter_mut()`, yielding `&mut T`s.
- A collection may provide some or all of these.

---
## Iterators

- Iterators provide syntactic sugar for for loops:

```rust
let values = vec![1, 2, 3, 4, 5];
{
    let result = match values.into_iter() {
        mut iter => loop {
            match iter.next() {
                Some(x) => { /* loop body */ },
                None => break,
            }
        },
    };
    result
}
```

- `into_iter()` is provided by the trait `IntoIterator`.
    - Automatically implemented by anything with the Trait `Iterator`.

---
## `IntoIterator`

```rust
pub trait IntoIterator {
    type Item;

    type IntoIter: Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

- You can implement `IntoIterator` on a `&T` to iterate over a collection by reference.
    - Or on `&mut T` to iterate by mutable reference.
- This allows this syntax:

```rust
let ones = vec![1, 1, 1, 1, 1, 1];

for one in &ones {
    // Doesn't move any values.
}
```

---
## Iterator Consumers

- Consumers operate on an iterator and return one or more values.
- There are like a billion of these, so let's look at a few.

---
## Preface: Type Transformations

- Many iterator manipulators take an `Iterator` and return some other type.
    - e.g. `map` returns a `Map`, `filter` returns a `Filter`.
- These types are just structs which themselves implement `Iterator`.
    - Don't worry about the internal state.
- The type transformations are used mostly to enforce type safety.

---
## `collect`

- `collect()` rolls a (lazy) iterator back into an actual collection.
- The target collection must define the `FromIterator` trait for the `Item`
  inside the `Iterator`.
- `collect()` sometimes needs a type hint to properly compile.
    - The output type can be practically any collection.

```rust
fn collect<B>(self) -> B where B: FromIterator<Self::Item>

let vs = vec![1,2,3,4];
// What type is this?
let set = vs.iter().collect();
// Hint to `collect` that we want a HashSet back.
// Note the lack of an explicit <i32>.
let set: HashSet<_> = vs.iter().collect();
// Alternate syntax! The "turbofish" ::<>
let set = vs.iter().collect::<HashSet<_>>();
```

---
## `fold`

```rust
fn fold<B, F>(self, init: B, f: F) -> B
    where F: FnMut(B, Self::Item) -> B;

let vs = vec![1,2,3,4,5];
let sum = vs.iter().fold(0, |acc, &x| acc + x);
assert_eq!(sum, 15);
```

- `fold` "folds up" an iterator into a single value.
    - Sometimes called `reduce` or `inject` in other languages.
- `fold` takes two arguments:
    - An initial value or "accumulator" (`acc` above) of type `B`.
    - A function that takes a `B` and the type inside the iterator (`Item`) and
        returns a `B`.
- Rust doesn't do tail-recursion, so `fold` is implemented iteratively.
    - [See here](https://github.com/rust-lang/rust/issues/217) if you're interested why.

---
## `filter`

```rust
fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where P: FnMut(&Self::Item) -> bool;
```

- `filter` takes a predicate function `P` and removes anything that doesn't pass
    the predicate.
- `filter` returns a `Filter<Self, P>`, so you need to `collect` it to get a new
    collection.

---
## `find` & `position`

```rust
fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where P: FnMut(Self::Item) -> bool;

fn position<P>(&mut self, predicate: P) -> Option<usize>
    where P: FnMut(Self::Item) -> bool;
```

- Try to find the first item in the iterator that matches the `predicate` function.
- `find` returns the item itself.
- `position` returns the item's index.
- On failure, both return a `None`.

---
## `skip`

```rust
fn skip(self, n: usize) -> Skip<Self>;
```
- Creates an iterator that skips its first `n` elements.

---
## `zip`

```rust
fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
    where U: IntoIterator;
```

- Takes two iterators and zips them into a single iterator.
- Invoked like `a.iter().zip(b.iter())`.
    - Returns pairs of items like `(ai, bi)`.
- The shorter iterator of the two wins for stopping iteration.

---
## `any` & `all`

```rust
fn any<F>(&mut self, f: F) -> bool
    where F: FnMut(Self::Item) -> bool;

fn all<F>(&mut self, f: F) -> bool
    where F: FnMut(Self::Item) -> bool;
```

- `any` tests if any element in the iterator matches the input function
- `all` tests all elements in the iterator match the input function
- Logical OR vs. logical AND.

---
## `enumerate`

```rust
fn enumerate(self) -> Enumerate<Self>;
```

- Want to iterate over a collection by item and index?
- Use `enumerate`!
- This iterator returns `(index, value)` pairs.
    - `index` is the `usize` index of `value` in the collection.

```rust
let data = [6, 1, 3, 9, 1];
for (i, value) in data.iter().enumerate() {
    println!("{} {}", i, value);
}
```

---
## Iterator Adapters

- Adapters operate on an iterator and return a new iterator.
- Adapters are often _lazy_  -- they don't evaluate unless you force them to!
- You must explicitly call some iterator consumer on an adapter or use it in a
    `for` loop to cause it to evaluate.

---
## `map`

```rust
fn map<B, F>(self, f: F) -> Map<Self, F>
    where F: FnMut(Self::Item) -> B;

let vs = vec![1,2,3,4,5];
let twice_vs: Vec<_> = vs.iter().map(|x| x * 2).collect();
```

- `map` takes a function and creates an iterator that calls the function on each
    element
- Abstractly, it takes a `Collection<A>` and a function of `A -> B` and
    returns a `Collection<B>`
    - (`Collection` is not a real type)

---
## `take` & `take_while`

```rust
fn take(self, n: usize) -> Take<Self>;

fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where P: FnMut(&Self::Item) -> bool;
```

- `take` creates an iterator that yields its first `n` elements.
- `take_while` takes a closure as an argument, and iterates until the closure
    returns `false`.
- Can be used on infinite ranges to produce finite enumerations:

```rust
for i in (0..).take(5) {
    println!("{}", i); // Prints 0 1 2 3 4
}
```

---
## `cloned`

```rust
fn cloned<'a, T>(self) -> Cloned<Self>
    where T: 'a + Clone, Self: Iterator<Item=&'a T>;
```

- Creates an iterator which calls `clone` on all of its elements.
- Abstracts the common pattern `vs.iter().map(|v| v.clone())`.
- Useful when you have an iterator over `&T`, but need one over `T`.

---
## `drain`

- Not actually an `Iterator` method, but is very similar.
- Calling `drain()` on a collection removes and returns some or all elements.
- e.g. `Vec::drain(&mut self, range: R)` removes and returns a range out of a vector.

---
## Iterators

- There are many more `Iterator` methods we didn't cover.
- Take a look at [the docs](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for the rest.

---
## Home assigment

- Review the covered material:
    - Review The Rust Programming Language book: https://doc.rust-lang.org/book/
    - Check examples in the Rust By Example book: https://doc.rust-lang.org/rust-by-example/
