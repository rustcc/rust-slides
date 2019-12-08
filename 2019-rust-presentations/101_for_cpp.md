title: Rust 101 for C++ developers
class: animation-fade
layout: true

<!--
This work is licensed under the Creative Commons Attribution-ShareAlike 4.0
International License. To view a copy of this license, visit
http://creativecommons.org/licenses/by-sa/4.0/ or send a letter to Creative
Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

.bottom-bar[
  {{title}}
]

---

class: impact

# {{title}}

.left-column[
Michal Vaner (michal.vaner@avast.com)
]

.right-column[
https://vorner.cz/rust-101-cpp.html
]

![](101_for_cpp/qr.svg)

---

# About me

* Senior software engineer II in Avast
* Using both C++ and Rust
  - Professionally and for fun
* Rust is currently favourite language
  - Maintaining several Rust libraries
  - Expect some bias in the talk
  - Won't tell you to Rewrite It In Rust
* Maybe both communities will hate me after today

???

* Contract: it is allowed to ask during the talk
* Talk about omni and backend
* Also python a bit and all bunch of other languages
* Self-appointed member of the Rust Evangelist Strike Force

---

# Why talk about Rust to C++ folks?

* C++ can do things impossible in other languages
  - „The last resort language“
  - Rust is a direct competitor for this niche, getting traction
* Rust is heavily inspired by C++
  - Both in positive and negative way
  - Probably the closest relative
  - Smaller investment for C++ people to learn
* Expanding the horizons
  - Techniques and patterns used in one can help in the other

???

* It's clear I want to talk about Rust, but why to C++ people?
* Last resort: allows doing high performance, crazy, insane and such
* Many companies now having a look (Mozilla, Dropbox, Microsoft)
* The most loved language on stack overflow 4times in a row
* C++ devs are one of the people coming to Rust, people from high-level
  languages needing fast programs another.

---

# About the talk

* To introduce what Rust is
* Mostly on the intuitive level
  - No time for fine rigorous details
  - Rust doesn't have a standard
* About real-life implementations of C++
  - The C++ standard allows weird implementations
* Will not *teach* Rust syntax
  - Maybe motivate to have a look yourself

???

* C++ on top of JVM, anyone?

---

class: impact

# First introductions

Let's meet the crabs

---

# Why I learned Rust

* Heard about Rust and it's safety guarantees
  - No segfaults, no data races
  - But without GC
* Pretty audacious claims
  - Not even GC languages protect against data races
  - 🦄
* Had a look to *disprove* the claims
  - And failed at that

???

* It's uncommon for something to fulfill the marketing claims
* Found other problems (and cheating), but the claims hold.

---

# The 10k miles overview

.left-column[
* Rust is *mostly* C++
  - If done today from scratch
* With Haskell's type system
* The best-practices *enforced* by the compiler
* Syntax & feature cleanup
* It wasn't *meant* that way from start
  - The motivation was safety, not C++ replacement
]

.right-column[

![Crab mascot](images/crab.svg)

]

???

* Unfortunately, it's getting some cruft in the syntax and features already too
* Story about similar to Go
  - Had channels, green threads, GC
  - Turned to true system language out of necessity (to support firefox) and to
    move to different niche.
  - Story about firefox and their attempt at parallel CSS engine

---

# Basic info

.left-column[
* Multi-paradigm
  - *Mostly* imperative
  - Allows functional, OOP, …
* Strong static typing
* Started by Graydon Hoare in 2006
  - Personal project
* Mozilla started to sponsor in 2009
* 1.0 released on May 15 2015
  - A new release every 6 weeks
* Community development
  - With semi-formal RFC process
]

.right-column[
```rust
fn main() {
    println!("Hello world");
}
```
]

???

* You've probably heard something, but if not...

---

class: impact

# Rust is similar to C++

You already know most of it without realizing

---

# Greatest Common Divisor, C++

```cpp
uint64_t gcd(uint64_t a, uint64_t b) {
    while (a != b) {
        if (a < b) {
            b -= a;
        } else {
            a -= b;
        }
    }
    return a;
}
```

???

* Yes, a more optimal way with division exists. This one is shorter.

---

# Greatest Common Divisor, Rust

```rust
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != b {
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    }
    a
}
```

???

* Small syntactical differences:
  - mut because stuff is immutable by default
  - types on the right, after `:`
  - Return value after `->` on the right
  - Last value is auto-returned (without semicolon)
  - Braces mandatory, parens around conditionals optional

---

# Execution & memory model

* The same for both languages
* Memory:
  - Explicit heap, static storage, threads with stacks
  - Stacks are non-relocable and continuous
  - No GC, semi-manual management
* Ahead of the time compilation to native code
* Threads map to OS threads
* Types compose without implicit indirection
  - Members are in-line
* Methods are just syntax sugar for functions
* Minimal run-time introspection

???

* Reminder: we are talking about common C++ implementations
* Rust is on top of LLVM, so a lot of that follows naturally

---

# Runtime

* Thin, used on-demand
* Pay only for what you use
* Possible to use without the standard library
* Contains:
  - Before-main initialization
  - Memory allocation
  - Stack unwinding
  - Runtime type information + dynamic dispatch

???

* no-std: Kernels, bootloaders, microprocessors, eBPF programs, programs on
  graphics cards...
  - Certainly makes life a bit harder

---

# Other similarities

* RAII
* Both dynamic and static method dispatch
  - Monomorphisation
* Compile-time meta programming
  - Different in each
* Both have undefined behaviour (not the same things)
  - An empty infinite loop is not UB in Rust
  - Only `unsafe` lets you write UB in Rust
* The same threading model
  - Memory orderings
* Compile times

???

* Compile-time meta programming through macros, procedural macros and traits.
* on UB:
  - The usual things ‒ pointers, memory accesses, data races in both
  - Non-existent variant or aliasing mutable reference is UB in rust
* Really long compile times

---

# Consequences

* Similar performance characteristics
  - In speed, RAM consumption and code size
* C ffi is very cheap
  - Allows building native extensions for other languages
* The same runtime tools work *mostly* out of the box
  - Valgrind, gdb, perf, heaptrack, …
  - Slight differences in name mangling
* Ability to combine Rust and C++
  - Thin glue code can be auto-generated
  - Allows for cross-language optimisations, like inlining

???

* Single-percent-point differences in benchmarks, going both ways
* Combining in eg. firefox

---

class: impact

# Rust is also different

There would be no point to it otherwise

---

# Primes, C++

```cpp
vector<uint64_t> cache;

for (uint64_t p = 2; ; p ++) {
  const auto boundary = find_if(cache.begin(), cache.end(),
                                [&] (auto c) { return c * c > p; });
  const auto divisor = find_if(cache.begin(), boundary,
                               [&] (auto c) { return p % c == 0; });

  if (divisor == boundary) {
    cout << p << endl;
    cache.push_back(p);
  }
}
```

???

* This prints primes, one by one, continuously until stopped
* Go through the code what it does and why

---

# Primes, Rust

```rust
let mut cache: Vec<u64> = Vec::new();

for p in 2u64.. {
    let has_divisor = cache
        .iter()
        .take_while(|&c| c * c <= p)
        .any(|c| p % c == 0);

    if !has_divisor {
        println!("{}", p);
        cache.push(p);
    }
}
```

???

* Technically almost equivalent, but not really visually similar, due to little
  things
* Bechmark:
  - Removed printing
  - Run until 100M
  - 1:16s vs 0:46
  - Compiled with `clang++ -O3` vs. `--release`
  - The difference probably due to double-scan in case of C++

---

class: broken

# Primes, „optimized“ C++

```cpp
vector<uint64_t> cache;
*auto boundary = cache.begin();

for (uint64_t p = 2; ; p ++) {
* boundary = find_if(boundary, cache.end(),
*                    [&] (auto c) { return c * c > p; });
  const auto divisor = find_if(cache.begin(), boundary,
                               [&] (auto c) { return p % c == 0; });

  if (divisor == boundary) {
    cout << p << endl;
    cache.push_back(p);
  }
}
```

???

* Let's optimize it, the boundary ever moves only forward, so we can cache it,
  right?
* Who sees what's wrong ‒ probably easy one
* Segfault, other nasty stuff
* Rust doesn't have natural place what to cache here in this code. If it had,
  the borrow checker would scream, because it would do a shared borrow (the
  boundary) + mut borrow (the push)

---

# Aside: Other options

* Manual `for` loop
* Use `lower_bound` to find the boundary
* C++-20 ranges
  - Similar to the Rust version

---

# Highlighted differences

* Syntax
* Drops 47 year old baggage of C compatibility
* Drops *a lot* of complexity
* Attitude to safety & human abilities
* Introduces very strong type system
* Compilation model
* Checked generics
* Error messages are first-class goal

???

* Syntax:
  - `variable: type`, type usually inferred without `auto`.
  - Functions with result on the right side
  - All items introduced by a keyword (fn, struct)
* No auto-conversions, not everything is int
  - C is strongly typed, but it's only type is int.
* Checked generics: first validates traits, then substitues.
  - Will fail to compile on errors even before instantiation

---

# Attitude towards the programmer

* To err is human
  - Especially on Monday morning before coffee ☕
* Cognitive bandwidth is limited
  - Save it for the problem, not language
* Compiler proves certain correctness guarantees
  - Frees some brain power of the programmer
  - Can't be broken by refactoring
* It's better to refuse correct program than compile an incorrect one
  - There are escape hatches

???

* C++ requires constant vigilance
* Gray area of programs the compiler can't prove right or wrong

---

# A word from the Father of Rust

*Basically I've an anxious, pessimist personality; most systems I try to build
are a reflection of how terrifying software-as-it-is-made feels to me. I'm
seeking peace and security amid a nightmare of chaos. I want to help programmers
sleep well, worry less.*

  *Graydon Hoare*

---

# Safe by default

* Borrow checker to enforce lifetimes
* Extra `unsafe` power enabled by a keyword
  - Most programs don't need to use
  - Mostly for building new abstractions or FFI
  - No UB without `unsafe`
* Variables default to immutable
* Visibility defaults to private
* All variables need to be initialized before use
* Encourages „pit of success“ APIs
  - Allows encoding certain properties into types
  - Example: HTTP body can be sent just once, after the headers
  - Example: `Mutex` contains protected data

???

* My favourite feature
* The devil is in the details...
  - You can write safe C++, with some effort
  - But in Rust you have to invest effort to write *unsafe* one.
* Fighting the borrow checker is a phase of learning Rust
  - Automatic prover
  - It is *usually* right ‒ discovers bugs in old C++ code when ported to Rust
  - One gets use to it
  - Teaches certain design patterns
* Probably missing something

---

# Mutex in C++

```cpp
class foo {
private:
    std::string data; // Protected by the mutex below
    std::mutex mutex;
public:
    void set_data(std::string new_data) {
        std::lock_guard<std::mutex> guard(mutex);
        data = std::move(new_data);
    }
    std::string get_data() const {
        std::lock_guard<std::mutex> guard(mutex);
        return data;
    }
}
```

???

* Note: this is mostly just an example. Imagine something more complex with
  business logic, possibly multiple fields, some might be read-only for the
  lifetime of the class and don't need the mutex, some do...
* Highlight: manual review needs to be done to make sure access really *is*
  protected by the mutex. In a long class full of business logic one might to
  overlook it and access directly.

---

# Mutex in Rust

```rust
use std::sync::Mutex;

struct Foo {
    Mutex<String> data;
}

impl Foo {
    fn set_data(&self, data: String) {
        let mut guard = self.data.lock().unwrap();
        *guard = data;
    }
    fn data(&self) -> String {
        self.data.lock().unwrap().clone()
    }
}
```

???

* The string lives inside the mutex.
* Adheres to Rust codestyle conventions (eg. getter without `get_`).
* Unwrap because of lock poisoning
* The setter would probably be written in a single command too, just showing off
  some syntax.
* Is 1:1 rewrite, including where move semantics or copying of things happen.
  See the `clone()` that's implicit in C++.
* Doesn't allow the access to the inner string without locking. In C++ we could
  make a mistake, for example overlook something during refactoring.
* I could break the protection, get a reference/pointer/access to the thing
  inside without the lock, but it would take like 2 slides to do so.

---

# Removal of complexity

* No inheritance
* No overloading
  - Results in simple scoping rules
* No default parameters
* Pointer arithmetics as a method
* No exceptions
  - Has panics which behave similarly
  - Strong exception guarantee is not mandated

???

* Something *like* overloading can be done with traits

--

+ Avoids hellfire combos
  - Combinations of features that do something *weird*
  - Like overloading & templates
  - Or inheritance & pointer to array decay

???

* Makes the code a bit more wordy
* Time lost by writing longer code is gained by not figuring out what went wrong
* But also less surprising and easier to read
  - Because things don't happen without a clue in the code

---

# A story with refactoring

* Found this in a header file:

```cpp
static const std::string answer = "42";
```

???

* Let's do a bit more practical intermezzo, demonstrating the above

--

* That looks *old style* and terrible and has other problems
* Let's just have an `unsigned` constant instead
* The compiler would show an error at every use
  - Compiler-driven refactoring

```cpp
static constexpr unsigned answer = 42;
```

---

# Except...

* No, this really old C pothole ⚠

```cpp
std::string msg = "The answer is " + answer;
std::cout << msg << std::endl;
```

--

* Yes, this produces a warning, but will you notice?

???

* And no, this is *not* a modern C++ style. But I didn't *intend* to write
  that.

---

# Type system

* Inspired by Haskell
* Algebraic types
  - Enums (with payload)
  - Something like tagged union
  - When matching, all variants must be handled
  - Convention: errors by `Result<T, E>`
* Traits
  - Similar to interfaces
  - Used for adding methods to types
  - Building block for generics
* No auto-conversions
  - Has coercions, but doesn't *change* the object

???

* But shoehorned into static dispatch of methods & imperative language
* Type elision allowed only inside functions, not in signatures
  - When looking at docs or headers, one wants to know the types, not the read
    the whole body to figure it out
  - Type elision works *backwards* too (eg. figuring the type of vector element
    by what is being inserted)
* Even `u64` and `usize` are different types and need manual conversion

---

class: impact

# Examples

Let's move to some practical things

---

# Read the whole file to memory

```rust
use std::io::{Error, Read};
use std::fs::{File, Path};

fn file_to_mem<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut f = File::open(path)?;
    let mut buff = Vec::new();
    f.read_to_end(&mut buff)?;
    Ok(buff)
}
```

???

* C++ version not showed, homework for the audience 😈
* Error handling:
  - Need to extract the Ok variant
  - Possible by pattern matching, methods, or the question mark → visible, but
    terse
* Generics, traits
  - On the input
  - The `read_to_end` is method of the `Read` trait, `File` implements that. But
    many other things do.
* Elision: The element of Vec on `Vec::new` is elided (from the `read_to_end`)
* The &mut make it visible the method may/will change the content

--

* Actually, [`std::fs::read`](https://doc.rust-lang.org/std/fs/fn.read.html) already exists

---

# Looking up an element, C++

```cpp
std::unordered_map<int64_t, std::string> map;

// ...

*if (const auto &elem = map.find(42); elem != map.end()) {
    std::cout << "Value of 42 is " << elem->second << std::endl;
} else {
    std::cout << "We don't have value of 42" << std::endl;
}
```

???

* It returns the same thing no matter if it found or not, lack of the element is
  signalled by special sentinel value.

---

class: broken

# Looking up an element, *broken* C++

```cpp
std::unordered_map<int64_t, std::string> map;

// ...

std::cout << "Value of 42 is " << map.find(42)->second;
```

???

* Who sees the problem?

--

* If 42 is missing, this is UB
  - Arbitrarily bad things may happen

---

# Looking up an element, Rust

```rust
use std::collections::HashMap;

let mut map: HashMap<i64, String> = HashMap::new();

// ...

*match map.get(&42) {
    Some(elem) => println!("Value of 42 is {}", elem),
    None => println!("We don't have value of 42"),
}
```

???

* The returned `Option<&String>` forces the programmer to acknowledge the
  possibility of missing element.
* Lifetimes watch problems with iterator invalidation
* There are other ways to write it, like `if let Some(...)` or monadic
  combinators...

---

# Looking up an element, *broken* Rust

```rust
use std::collections::HashMap;

let mut map: HashMap<i64, String> = HashMap::new();

// ...

println!("The value of 42 is {}", map.get(&42).unwrap());
```

* If value is missing, it panics in *defined* manner
* The `unwrap` is visible in the code and conveys the intention

---

# Moving a vector, C++

```cpp
void consume_vector(std::vector<int> data) {
    // ...
}

int main() {
    std::vector<int> v;
    v.push_back(10);
    v.push_back(20);
    // A new vector is created, takes over v's allocation.
    // v becomes empty, but still exists.
    consume_vector(std::move(v));
    // You still can do this.
    v.push_back(30);
    return 0;
    // Destructor of v runs here.
}
```

???

* Actually, the standard allows making a copy or leaving the old vector
  non-empty...

---

# Moving a vector, Rust

```rust
fn consume_vector(data: vec<isize>) {
    // ...
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    // No new vector is created. v's stack image is relocated
    // to a different address, still pointing to the old data.
    consume_vector(v);
    // Here, v no longer exists. Trying to access it is compilation
    // error.
    // v.push(30);
    // Therefore, no destructor runs here.
}
```

???

* Move semantics are the default
  - The move is true move, not just swapping/gutting

---

# Moving a vector, idiomatic Rust

* There's a macro to create a vector

```rust
fn consume_vector(data: vec<isize>) {
    // ...
}

fn main() {
    consume_vector(vec![10, 20]);
}
```

???

* No mutability and creating the vector by pushing one by one. Also possibly
  faster.

---

class: impact

# Time savers

All the annoyances one doesn't have to care about

???

* A language is not only about the language itself
  - Libraries
  - Tooling
  - Community

---

# Ecosystem & tooling

* `cargo` ‒ Compilation & package manager
  - Central repository of packages
  - Easy to use libraries
  - Common to use many small libraries
* Common tooling out of the box
  - Writing & running tests
  - Documentation generation
  - Documentation code examples are compiled and tested
* Allows building further tools
  - `cargo update` & `cargo outdated`
  - `cargo audit`
  - `cargo bloat`

???

* Describe what each one does

---

# Example

```rust
/// Sums two numbers
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_small() {
        assert_eq!(4, sum(2, 2));
    }
}
```

???

* `cargo doc --open` will show API documentation
* `cargo test` will run the tests

---

# Cargo audit

```
error: Vulnerable crates found!

ID:       RUSTSEC-2019-0024
Crate:    rustsec-example-crate
Version:  0.0.1
Date:     2019-10-08
URL:      https://rustsec.org/advisories/RUSTSEC-2019-0024
Title:    Test advisory with associated example crate
Solution:  upgrade to >= 1.0.0
Dependency tree: 
rustsec-example-crate 0.0.1
└── broken 0.1.0

error: 1 vulnerability found!
```

???

* Show the advisory page?
  - https://rustsec.org/advisories/RUSTSEC-2019-0024

---

# Commonly accepted conventions

* Naming style is warned about by compiler
* Official coding style
  - `rustfmt` can reformat the code
* Naming guidelines & API guidelines

+ Helps reading some else's code
+ Saves time arguing about bikeshedding matters

???

* In C++, everyone seems to have their own style

---

# Learning materials

* [The Rust Book](https://doc.rust-lang.org/stable/book/)
* [Rust For C++ programmers](https://github.com/nrc/r4cppp)
* [Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)
  - Because linked lists in Rust are *hard*
* [The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)
  - About the advanced and unsafe parts

???

* Describe what each of the sources is about.

+ Come work to Avast

---

# Rust...

* ...can be as fast as C++
* ...can be combined with C++
* ...tries to prevent common bugs
* ...will make you a better programmer
  - Even if you don't decide to use it in the end

???

* A conclusion
* Time for more questions

---

class: impact

# Happy learning!

And many new Rustaceans
