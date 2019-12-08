# Exercises

## Exercise 1 (understanding ownership)

* Why don't Box (or other owning types) need a lifetime parameter?
* Why is it an ownership graph and not a tree?
* What is at the roots of the ownership graph?
* How does ownership relate to mutability?

## Exercise 2 (types and control flow)

Write the following functions as succinctly as you can (you'll want to check the
[std docs](https://doc.rust-lang.org/std/index.html)):

[Playground](https://play.rust-lang.org/?gist=49e62fcba1adfdbe2107fcf20a997b1b&version=nightly&mode=debug&edition=2018)

```rust
fn foo(input: Option<i32>) -> Option<i32> {
    if input.is_none() {
        return None;
    }

    let input = input.unwrap();
    if input < 0 {
        return None;
    }
    Some(input)
}

fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    match foo(input) {
        Some(n) => Ok(n),
        None => Err(ErrNegative),
    }
}

#[derive(Debug)]
struct ErrNegative;

fn main() {
    println!("{:?}", foo(Some(0)));
    println!("{:?}", foo(None));
    println!("{:?}", bar(Some(42)));
    println!("{:?}", bar(None));
}
```

## Exercise 3 (iteration)

Lower the following program to a simpler one without `while let` by using `loop`,
`match`, and `break`.

[Playground](https://play.rust-lang.org/?gist=5ae0ae4d5fdad1a90fc49c8b91885507&version=nightly&mode=debug&edition=2018)

```rust
fn main() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}
```


## Exercise 4 (error handling)

Convert the first three functions to an error handling strategy, rather than
a panicking strategy.

[Playground](https://play.rust-lang.org/?gist=fa7f507c00a7f720676e1d7a24076de3&version=nightly&mode=debug&edition=2018)

```rust
// Remove the `expect`s from these two functions
fn read_config() -> ConfigFile {
    let file = read_file().expect("could not open file");

    write_to_log().expect("could not write to log file");
    file
}

impl Server {
    fn startup(self) -> ListeningServer {
        let config = read_config();
        let port = open_port().expect("could not open port");
        self.configure(config, port)
    }
}

fn main() {
    let server = Server::new();
    let server = server.startup();
    while let Some(_packet) = server.listen() {
        // ...
    }
}

// Helper functions and types, don't change these
fn write_to_log() -> Result<(), DiskError> { Ok(()) }
fn open_port() -> Result<Port, NetworkError> { Ok(Port) }
fn read_file() -> Result<ConfigFile, DiskError> { Ok(ConfigFile) }

struct Server;
struct Port;
#[derive(Debug)]
struct DiskError;
#[derive(Debug)]
struct NetworkError;
struct ConfigFile;
struct ListeningServer;

impl Server {
    fn new() -> Server { Server }
    fn configure(self, _c: ConfigFile, _p: Port) -> ListeningServer { ListeningServer }
}

impl ListeningServer {
    fn listen(&self) -> Option<()> { None }
}
```


## Exercise 5 (abstraction with traits)

Find a crate (your own or somebody else's) and see if you can find a place where
you can refactor concrete types using traits.

Does the code look better?

Is the code more extensible or testable?
