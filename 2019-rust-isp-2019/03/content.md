# I/O & Threading

### Rust ISP 2019 Lecture 01
Based on: [CIS 198 slides](https://github.com/cis198-2016s/slides)

Artyom Pavlov, 2019.

---
# I/O

---
## Traits!

```rust
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    // Other methods implemented in terms of read().
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // Other methods implemented in terms of write() and flush().
}
```

- Standard IO traits implemented for a variety of types:
    - `File`s, `TcpStream`s, `Vec<T>`s, `&[u8]`s.
- Careful: return types are `std::io::Result`, not `std::Result`!
    - `type Result<T> = Result<T, std::io::Error>;`

---
## `std::io::Read`

```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("foo.txt"));
let mut buffer = [0; 10];

// read up to 10 bytes
try!(f.read(&mut buffer));
```

- `buffer` is an array, so the max length to read is encoded into the type.
- `read` returns the number of bytes read, or an `Err` specifying the problem.
    - A return value of `Ok(n)` guarantees that `n <= buf.len()`.
    - It can be `0`, if the reader is empty.

---
## Ways of Reading

```rust
/// Required.
fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

/// Reads to end of the Read object.
fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>

/// Reads to end of the Read object into a String.
fn read_to_string(&mut self, buf: &mut String) -> Result<usize>

/// Reads exactly the length of the buffer, or throws an error.
fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>
```

- `Read` provides a few different ways to read into a variety of buffers.
    - Default implementations are provided for them using `read`.
- Notice the different type signatures.

---
## Reading Iterators

```rust
fn bytes(self) -> Bytes<Self> where Self: Sized

// Unstable!
fn chars(self) -> Bytes<Self> where Self: Sized
```

- `bytes` transforms some `Read` into an iterator which yields byte-by-byte.
- The associated `Item` is `Result<u8>`.
    - So the type returned from calling `next()` on the iterator is
      `Option<Result<u8>>`.
    - Hitting an `EOF` corresponds to `None`.

- `chars` does the same, and will try to interpret the reader's contents as a
  UTF-8 character sequence.
    - Unstable; Rust team is not currently sure what the semantics of this
      should be. See issue [#27802][].

[#27802]: https://github.com/rust-lang/rust/issues/27802

---
## Iterator Adaptors

```rust
fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    where Self: Sized
```
- `chain` takes a second reader as input, and returns an iterator over all bytes
  from `self`, then `next`.

```rust
fn take<R: Read>(self, limit: u64) -> Take<Self>
    where Self: Sized
```
- `take` creates an iterator which is limited to the first `limit` bytes of the
  reader.

---
## `std::io::Write`

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    // Other methods omitted.
}
```

- `Write` is a trait with two required methods, `write()` and `flush()`
    - Like `Read`, it provides other default methods implemented in terms of
      these.
- `write` (attempts to) write to the buffer and returns the number of bytes
  written (or queued).
- `flush` ensures that all written data has been pushed to the target.
    - Writes may be queued up, for optimization.
    - Returns `Err` if not all queued bytes can be written successfully.
---
## Writing

```rust
let mut buffer = try!(File::create("foo.txt"));

try!(buffer.write("Hello, Ferris!"));
```

---
## Writing Methods

```rust
/// Attempts to write entire buffer into self.
fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }

/// Writes a formatted string into self.
/// Don't call this directly, use `write!` instead.
fn write_fmt(&mut self, fmt: Arguments) -> Result<()> { ... }

/// Borrows self by mutable reference.
fn by_ref(&mut self) -> &mut Self where Self: Sized { ... }
```

---
## `write!`

- Actually using writers can be kind of clumsy when you're doing a general
  application.
    - Especially if you need to format your output.
- The `write!` macro provides string formatting by abstracting over
  `write_fmt`.
- Returns a `Result`.

```rust
let mut buf = try!(File::create("foo.txt"));

write!(buf, "Hello {}!", "Ferris").unwrap();
```

---
## IO Buffering

- IO operations are really slow.
- Like, _really_ slow:

```rust
TODO: demonstrate how slow IO is.
```

- Why?

---
## IO Buffering

- Your running program has very few privileges.
- Reads are done through the operating system (via system call).
    - Your program will do a _context switch_, temporarily stopping execution so
      the OS can gather input and relay it to your program.
    - This is veeeery slow.
- Doing a lot of reads in rapid succession suffers hugely if you make a system
  call on every operation.
    - Solve this with buffers!
    - Read a huge chunk at once, store it in a buffer, then access it
      little-by-little as your program needs.
- Exact same story with writes.

---
## BufReader

```rust
fn new(inner: R) -> BufReader<R>;
```
```rust
let mut f = try!(File::open("foo.txt"));
let buffered_reader = BufReader::new(f);
```

- `BufReader` is a struct that adds buffering to *any* reader.
- `BufReader` itself implements `Read`, so you can use it transparently.

---
## BufReader

- `BufReader` also implements a separate interface `BufRead`.

```rust
pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8]>;
    fn consume(&mut self, amt: usize);

    // Other optional methods omitted.
}
```

---
## BufReader

- Because `BufReader` has access to a lot of data that has not technically been
  read by your program, it can do more interesting things.
- It defines two alternative methods of reading from your input, reading up
  until a certain byte has been reached.

```rust
fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>)
    -> Result<usize> { ... }
fn read_line(&mut self, buf: &mut String)
    -> Result<usize> { ... }
```

- It also defines two iterators.

```rust
fn split(self, byte: u8)
    -> Split<Self> where Self: Sized { ... }
fn lines(self)
    -> Lines<Self> where Self: Sized { ... }
```
---
## BufWriter

- `BufWriter` does the same thing, wrapping around writers.

```rust
let f = try!(File::create("foo.txt"));
let mut writer = BufWriter::new(f);
try!(buffer.write(b"Hello world"));
```

- `BufWriter` doesn't implement a second interface like `BufReader` does.
- Instead, it just caches all writes until the `BufWriter` goes out of scope,
  then writes them all at once.

---
## `StdIn`

```rust
let mut buffer = String::new();

try!(io::stdin().read_line(&mut buffer));
```

- This is a very typical way of reading from standard input (terminal input).
- `io::stdin()` returns a value of `struct StdIn`.
- `stdin` implements `read_line` directly, instead of using `BufRead`.

---
## `StdInLock`

- A "lock" on standard input means only that current instance of `StdIn` can
  read from the terminal.
    - So no two threads can read from standard input at the same time.
- All `read` methods call `self.lock()` internally.
- You can also create a `StdInLock` explicitly with the `stdin::lock()` method.

```rust
let lock: io::StdInLock = io::stdin().lock();
```

- A `StdInLock` instance implements `Read` and `BufRead`, so you can call any of
  the methods defined by those traits.

---
## `StdOut`

- Similar to `StdIn` but interfaces with standard output instead.
- Directly implements `Write`.
- You don't typically use `stdout` directly.
    - Prefer `print!` or `println!` instead, which provide string formatting.
- You can also explicitly `lock` standard out with `stdout::lock()`.

---
## Special IO Structs

- `repeat(byte: u8)`: A reader which will infinitely yield the specified byte.
    - It will always fill the provided buffer.
- `sink()`: "A writer which will move data into the void."
- `empty()`: A reader which will always return `Ok(0)`.
- `copy(reader: &mut R, writer: &mut W) -> Result<u64>`: copies all bytes from
  the reader into the writer.

---
# Serialization

---
## [Serde](https://serde.rs/)

- **Ser**ialization/**De**serialization.
- Serde is built on Rust's powerful trait system. A data structure that knows how to serialize and deserialize itself is one that implements Serde's `Serialize` and `Deserialize` traits (or uses Serde's derive attribute to automatically generate implementations at compile time).
- Supports many formats, like: JSON, bincode, MessagePack, XML, YAML, TOML, etc.

---
## Serd example

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```


---
## Networking

---
### Sockets

- Most of this section is preface.
- We're not actually going to cover most of what's in it directly.

---
### Sockets

- A basic way to send data over the network.
    - Not to be confused with IPC sockets, which are a Unix thing.
- Abstractly, a socket is just a channel that can send and/or receive data over
    some network.
- Many layers of socket-programming providers:
    - Operating system-provided system calls.
    - Low-level/low-abstraction programming language standard library.
    - Higher-level networking libraries or libraries handling a specific
      protocol (e.g. HTTP).
- Usually, you won't use sockets directly unless you want to do some
    low-level networking.
- Two general types: datagram & stream.

---
### Datagram Sockets (UDP)

- **U**ser **D**atagram **P**rotocol sockets
- Stateless: no connection to establish with another network device.
    - Simply send data to a destination IP and port, and assume they're
        listening.
- "At least once" delivery.
    - Packets are not guaranteed to be delivered in order.
    - Packets may be received more than once.
- Traditionally implement two methods:
    - send_to(addr) -- sends data over the socket to the specified address
    - recv_from() -- listens for data being sent to the socket

---
### `std::net::UdpSocket`

```rust
// Try to bind a UDP socket
let mut socket = try!(UdpSocket::bind("127.0.0.1:34254"));

// Try to receive data from the socket we've bound
let mut buf = [0; 10];
let (amt, src) = try!(socket.recv_from(&mut buf));

// Send a reply to the socket we just received data from
let buf = &mut buf[..amt];
buf.reverse();
try!(socket.send_to(buf, &src));

// Close the socket
drop(socket);
```

&sup1;Taken from the Rust docs.

---
### Stream Sockets (TCP)

- "This is where the drugs kick in" - Matt Blaze on TCP sockets
- **T**ransmission **C**ontrol **P**rotocol sockets
- Stateful: require a connection to be established and acknowledged between two
    clients (using SYN packet).

    - Connection must also be explicitly closed.
- Packets are delivered in-order, exactly once.
    - Achieved via packet sequence numbers.
- Packets have delivery acknowledgement (ACK packet).
- Generally two types of TCP socket:
    - TCP listeners: listen for data
    - TCP streams: send data

---
### `std::net::TcpStream`

- A TCP stream between a local socket and a remote socket.

```rust
// Create a TCP connection
let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

// Uses std::io::{Read, Write}

// Try to write a byte to the stream
let write_result = stream.write(&[1]);

// Read from the stream into buf
let mut buf = [0; 128];
let read_result = stream.read(&mut buf);

// ...
// Socket gets automatically closed when it goes out of scope
```

---
### `std::net::TcpListener`

- A TCP socket server.

```rust
let listener = TcpListener::bind("127.0.0.1:80").unwrap();

fn handle_client(stream: TcpStream) { /* ... */  }

// Accept connections and process them,
// spawning a new thread for each one.
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            thread::spawn(move|| {
                // connection succeeded
                handle_client(stream)
            });
        }
        Err(e) => { /* connection failed */ }
    }
}

// close the socket server
drop(listener);
```

---
### `SocketAddr`

- A socket address representation.
- May be either IPv4 or IPv6.
- Easily created using...

---
### `ToSocketAddrs`

```rust
pub trait ToSocketAddrs {
    type Iter: Iterator<Item=SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter>;
}
```

- A trait for objects which can be converted into `SocketAddr` values.
- Methods like `TcpStream::connect(addr: A)` specify that `A: ToSocketAddr`.
    - This makes it easier to specify what may be converted to a socket
        address-like object.
- See the docs for the full specification.

---
### HTTP
- Request-response text-based protocol which works on top of TCP.
- Foundation of the moddern Web.
- Check-out [arewewebyet.org/](https://www.arewewebyet.org/) for list of crates which can be used for Web development in Rust.

---
#Concurrency

---
## What is Concurrency?

- One program with multiple threads of execution running at the same time.
- Threads can share data without communication overhead.
    - (networking, inter-process communication channels, etc).
- Threads are more lightweight than individual processes.
    - No large OS context switch when switching between threads.

---
## What is a Thread?

- A context in which instructions are being executed.
- References to some data (which may or may not be shared).
- A set of register values, a stack, and some other information about the
    current execution context (low-level).

---
## Threads

- Conceptually, every program has at least one thread.
- There is a thread scheduler which manages the execution of these threads.
    - It can arbitrarily decide when to run any thread.
- Programs can create and start new threads, which will be picked up by the
  scheduler.

---
## Concurrent Execution

- Take these two simple programs, written in pseudo-Rust
    (ignoring ownership semantics):

```rust
let mut x = 0;

fn foo() {
    let mut y = &mut x;
    *y = 1;
    println!("{}", *y); // foo expects 1
}

fn bar() {
    let mut z = &mut x;
    *z = 2;
    println!("{}", *z); // bar expects 2
}
```

---
### Instruction Interleaving

- Imagine two threads: one executing `foo`, one executing `bar`.
- The scheduler can interleave instructions however it wants.
- Thus, the above programs may be executed like this:

```rust
/* foo */ let mut y = &mut x;
/* foo */ *y = 1;
/* foo */ println!("{}", *y); // foo expects 1
          // => 1
/* bar */ let mut z = &mut x;
/* bar */ *z = 2;
/* bar */ println!("{}", *z); // bar expects 2
          // => 2
```
- ...and everything works as expected.

---
### Instruction Interleaving

- However, there is no guarantee that execution happens in that order every time,
    or at all!
- We need some mechanisms to ensure that events happen in an order that produces
    the expected results.
- Otherwise, `foo` and `bar` may be interleaved arbitrarily, causing unexpected
    results:

```rust
/* bar */ let mut z = &mut x;
/* bar */ *z = 2;
/* foo */ let mut y = &mut x;
/* foo */ *y = 1;
/* bar */ println!("{}", *z); // bar expects 2
          // => 1
/* foo */ println!("{}", *y); // foo expects 1
          // => 1
```

---
## Why is concurrency hard?

- **Sharing data:** What if two threads try to write to the same piece of
  data at the same time?
    - Writing to `x` in the previous example.
- **Data races:** The behavior of the same piece of code might change depending
  on when exactly it executes.
    - Reading from `x` in the previous example.

---
## Why is concurrency hard?

- **Synchronization:** How can I be sure all of my threads see the correct world view?
    - A series of threads shares the same buffer. Each thread `i` writes to
      `buffer[i]`, then tries to read from the entire buffer to decide its next
      action.
    - When sending data between threads, how can you be sure the other thread
        receives the data at the right point in execution?
- **Deadlock:** How can you safely share resources across threads and ensure
    threads don't lock each other out of data access?

---
## Deadlock

- A deadlock occurs when multiple threads want to access some shared resources,
    but end up creating a state in which no one is able to access anything.
- There are four preconditions for deadlock:
    - Mutual exclusion: One resource is locked in a non-sharable mode.
    - Resource holding: A thread holds a resource and asks for more resources,
        which are held by other threads.
    - No preemption: A resource can only be released voluntarily by its holder.
    - Circular waiting: A cycle of waiting on resources from other threads
        exists.
- To avoid deadlock, we only have to remove _one_ precondition.

---
## Dining Philosophers

- An oddly-named classical problem for illustrating deadlock.
- N philosophers sit in a circle and want to alternate between eating and
    thinking.
- Each philosopher needs two forks to eat. There are `N` forks, one between
  every pair of philosophers.
- Algorithm:
    - A philosopher picks up their left fork. (Acquire a resource lock)
    - They pick up their right fork. (Acquire a resource lock)
    - They eat. (Use the resource)
    - They put both forks down. (Release resource locks)

---
## Dining Philosophers

- What happens when we do this?
    - Let N = 3, for simplicity.
- Philosopher 1 picks up their left fork.
- Philosopher 2 picks up their left fork.
- Philosopher 3 picks up their left fork.
- Philosophers 1, 2, and 3 all try to pick up their right fork, but get stuck,
    since all forks are taken!

---
## Dining Philosophers

- A better algorithm?
- We'll revisit this at the end of the lecture.

---
## Rust Threads

- Rust's standard library contains a threading library, `std::thread`.
    - Other threading models have been added and removed over time.
    - The Rust "runtime" was been removed.
- Each thread in Rust has its own stack and local state.
- In Rust, you define the behavior of a thread with a closure:

```rust
use std::thread;

thread::spawn(|| {
    println!("Hello, world!");
});
```

---
## Thread Handlers

- `thread::spawn` returns a thread handler of type `JoinHandler`.

```rust
use std::thread;

let handle = thread::spawn(|| {
    "Hello, world!"
});

println!("{:?}", handle.join().unwrap());
// => Ok("Hello, world!")
```
- `join()` will block until the thread has terminated.
- `join()` returns an `Ok` of the thread's final expression (or return
  value), or an `Err` of the thread's `panic!` value.

---
## `std::thread::JoinHandler`

- A thread is detached when its handler is dropped.
- Cannot be cloned; only one variable has the permission to join a thread.

---
## `panic!`

- Thread panic is unrecoverable from _within_ the panicking thread.
- Rust threads `panic!` independently of the thread that created them.
    - _Only_ the thread that panics will crash.
    - The thread will unwind its stack, cleaning up resources.
    - The message passed to `panic!` can be read from other threads.
- If the main thread panics or otherwise ends, all other threads will be shut down.
    - The main thread can choose to wait for all threads to finish before finishing itself.

???

Freeing allocations, running destructors.

---
## `std::thread::Thread`

- The currently running thread can stop itself with `thread::park()`.
- `Thread`s can be unparked with `.unpark()`.

```rust
use std::thread;

let handle = thread::spawn(|| {
    thread::park();
    println!("Good morning!");
});
println!("Good night!");
handle.thread().unpark();
```

- A `JoinHandler` provides `.thread()` to get that thread's `Thread`.
- You can access the currently running `Thread` with `thread::current()`.

---
## Many Threads

- You can create many threads at once:

```rust
use std::thread;

for i in 0..10 {
    thread::spawn(|| {
        println!("I'm first!");
    });
}
```

---
## Many Threads

- Passing ownership of a variable into a thread works just like the rest of the
  ownership model:

```rust
use std::thread;

for i in 0..10 {
    thread::spawn(|| {
        println!("I'm #{}!", i);
    });
}
// Error!
// closure may outlive the current function, but it borrows `i`,
// which is owned by the current function
```
- ...including having to obey closure laws.

---
## Many Threads

- The closure needs to own `i`.
- Fix: Use `move` to make a movable closure that takes ownership of its scope.

```rust
use std::thread;

for i in 0..10 {
    thread::spawn(move || {
        println!("I'm #{}!", i);
    });
}
```

???

Same thing as when you return a closure from a function.

---
## `Send` and `Sync`

- Rust's type system includes traits for enforcing certain concurrency
  guarantees.
- `Send`: a type can be safely transferred between threads.
- `Sync`: a type can be safely shared (with references) between threads.
- Both `Send` and `Sync` are marker traits, which don't implement any methods.

---
## `Send`

```rust
pub unsafe trait Send { }
```

- A `Send` type may have its ownership tranferred across threads.
- Not implementing `Send` enforces that a type _may not_ leave its original thread.
    - e.g. a C-like raw pointer, which might point to data aliased by another
      (mutable) raw pointer that could modify it in a thread-unsafe way.

---
## `Sync`

```rust
pub unsafe trait Sync { }
```

- A `Sync` type cannot introduce memory unsafety when used across multiple
  threads (via shared references).
- All primitive types are `Sync`; all aggregate types containing only items that
  are `Sync` are also `Sync`.
    - Immutable types (`&T`) and simple inherited mutability (`Box<T>`) are
      `Sync`.
    - Actually, all types without interior mutability are inherently (and automatically) `Sync`.

---
## `Sync`

- A type `T` is `Sync` if `&T` is thread-safe.
    - `T` is thread safe if there is no possibility of data races when passing
      `&T` references between threads
- Consequently, `&mut T` is also `Sync` if `T` is `Sync`.
    - An `&mut T` stored in an aliasable reference (an `& &mut T`) becomes
      immutable and has no risk of data races.
- Types like `Cell` are not `Sync` because they allow their contents to be
    mutated even when in an immutable, aliasable slot.
    - The contents of an `&Cell<T>` could be mutated, even when shared across
        threads.

---
## Unsafety

- Both `Send` and `Sync` are `unsafe` to implement, even though they have no
    required functionality.
- Marking a trait as `unsafe` indicates that the implementation of the trait
    must be trusted to uphold the trait's guarantees.
    - The guarantees the trait makes must be assumed to hold, regardless of
        whether it does or not.
- `Send` and `Sync` are unsafe because thread safety is not a property that can
    be guaranteed by Rust's safety checks.
    - Thread unsafety can only be 100% prevented by _not using threads_.
- `Send` and `Sync` require a level of trust that safe code alone cannot provide.

---
## Derivation

- `Send` is auto-derived for all types whose members are all `Sync`.
- Symmetrically, `Sync` is auto-derived for all types whose members are all
  `Send`.
- They can be trivially `impl`ed, since they require no members:

```rust
unsafe impl Send for Foo {}
unsafe impl Sync for Foo {}
```

---
## Derivation

- If you need to remove an automatic derivation, it's possible.
    - Types which _appear_ `Sync` but _aren't_ (due to `unsafe`
      implementation) must be marked explicitly.
    - Doing this requires so-called
      ["OIBITs": "opt-in builtin traits"](https://github.com/rust-lang/rust/issues/13231).

```rust
#![feature(optin_builtin_traits)]

impl !Send for Foo {}
impl !Sync for Foo {}
```

> _The acronym "OIBIT", while quite fun to say, is quite the anachronism. It
> stands for "opt-in builtin trait". But in fact, Send and Sync are neither
> opt-in (rather, they are opt-out) nor builtin (rather, they are defined in
> the standard library). It seems clear that it should be changed._
> [&mdash;nikomatsakis](https://internals.rust-lang.org/t/pre-rfc-renaming-oibits-and-changing-their-declaration-syntax/3086)

---
## Sharing Thread State

- The following code looks like it works, but doesn't compile:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    thread::sleep(Duration::from_millis(50));
}

// error: capture of moved value: `data`
//        data[i] += 1;
//        ^~~~
```

---
## Sharing Thread State

- If each thread were to take a reference to `data`, and then independently
    take ownership of `data`, `data` would have multiple owners!
- In order to share `data`, we need some type we can share safely between threads.
    - In other words, we need some type that is `Sync`.

---
### `std::sync::Arc<T>`

- One solution: `Arc<T>`, an **A**tomic **R**eference-**C**ounted pointer!
    - Pretty much just like an `Rc`, but is thread-safe due to atomic reference counting.
    - Also has a corresponding `Weak` variant.
- Let's see this in action...

---
## Sharing Thread State

- This looks like it works, right?

```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        let data = data.clone(); // Increment `data`'s ref count
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

---
## Sharing Thread State

- Unfortunately, not quite.

```
error: cannot borrow immutable borrowed content as mutable
                   data[i] += 1;
                   ^~~~
```

- Like `Rc`, `Arc` has no interior mutability.
- Its contents cannot be mutated unless it has one strong reference and no weak
  references.
    - Cloning the `Arc` naturally prohibits doing this.

---
## Many Threads

- `Arc<T>` assumes its contents must be `Sync` as well, so we can't mutate
    anything inside the `Arc`. :(
- What could we do to solve this?
    - We can't use a `RefCell`, since already know these aren't thread-safe.
    - Instead, we need to use a `Mutex<T>`.

---
## Mutexes

- Short for **Mut**ual **Ex**clusion.
- Conceptually, a mutex ensures that a value can only ever be accessed by one
    thread at a time.
- In order to access data guarded by a mutex, you need to acquire the
    mutex's lock.
- If someone else currently has the lock, you can either give up and try again
    later, or block (wait) until the lock is available.

---
## `std::sync::Mutex<T>`

- When a value is wrappted in a `Mutex`, you must call `lock` on the `Mutex` to
    get access to the value inside. This method returns a `LockResult`.
- If the mutex is locked, the method will block until the mutex becomes unlocked.
    - If you don't want to block, call `try_lock` instead.
- When the mutex unlocks, `lock` returns a `MutexGuard`, which you can dereference
    to access the `T` inside.

---
## Mutex Poisoning 🐍

- If a thread acquires a mutex lock and then panics, the mutex is considered
    _poisoned_, as the lock was never released.
- This is why `lock()` returns a `LockResult`.
    - `Ok(MutexGuard)`: the mutex was not poisoned and may be used.
    - `Err(PoisonError<MutexGuard>)`: a poisoned mutex.
- If you determine that a mutex has been poisoned, you may still access the
    underlying guard by calling `into_inner()`, `get_ref()`, or `get_mut()` on the `PoisonError`.
    - This may result in accessing incorrect data, depending on what the
        poisoning thread was doing.

---
## Sharing Thread State

- Back to our example:

```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone(); // Increment `data`'s ref count
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

---
## Sharing Thread State

- At the end of this example, we put the main thread to sleep for 50ms to wait
  for all threads to finish executing.
    - This is totally arbitrary; what if each thread takes much longer than
        that?
- We have no way to synchronize our threads' executions, or to know when they've
    all finished!

---
## Channels

- Channels are one way to synchronize threads.
- Channels allow passing messages between threads.
- Can be used to signal to other threads that some data is ready, some event has
  happened, etc.

---
## `std::sync::mpsc`

- **M**ulti-**P**roducer, **S**ingle-**C**onsumer communication primitives.
- Three main types:
    - `Sender`
    - `SyncSender`
    - `Receiver`
- `Sender` or `SyncSender` can be used to send data to a `Receiver`
- Sender types may be cloned and given to multiple threads to create *multiple producers*.
    - However, Receivers cannot be cloned (*single consumer*).

---
## `std::sync::mpsc`

- A linked `(Sender<T>, Receiver<T>)` pair may be created using the
    `channel<T>()` function.
- `Sender` is an _asynchronous_ channel.
- Sending data across the channel will never block the sending thread, since the
    channel is asynchronous.
    - `Sender` has a conceptually-infinite buffer.
- Trying to receive data from the `Receiver` will block the receiving thread until data arrives.

---
## `std::sync::mpsc`

```rust
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(i).unwrap();
        });
    }
    drop(tx);

    let mut acc = 0;
    while let Ok(i) = rx.recv() {
        acc += i;
    }
    assert_eq!(acc, 45);
}
```

---
## `std::sync::mpsc`

- A linked `(SyncSender<T>, Receiver<T>)` pair may be created using the
    `sync_channel<T>()` function.
- `SyncSender` is, naturally, synchronized.
- `SyncSender` _does_ block when you send a message.
    - `SyncSender` has a bounded buffer, and will block until there is buffer
        space available.
- Since this `Receiver` is the same as the one we got from `channel()`, it will
    obviously also block when it tries to receive data.

---
## `std::sync::mpsc`

- All channel send/receive operations return a `Result`, where an error
    indicates the other half of the channel "hung up" (was dropped).
- Once a channel becomes disconnected, it cannot be reconnected.

---
## So, is concurrency still hard?

- **Sharing data is hard:** Share data with `Send`, `Sync`, and `Arc`
- **Data races:** `Sync`
- **Synchronization:** Communication using channels.
- **Deadlock:** ??

---
## Dining Philosophers

- This illustrates a problem with the concurrency primitives we've seen so far:
    - While they can keep our code safe, they do not guard against all possible
      logical bugs.
    - These are not "magic bullet" concepts.
- Solutions?
- There are many:
    - The final philosopher pick up their forks in the opposite order.
    - A token may be passed between philosophers, and a philosopher may only try
      to pick up forks when they have the token.
