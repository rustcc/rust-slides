# Solutions to exercises

The exercises can be found in the slides and in [exercises.md](exercises.md).

## Exercise 1 (understanding ownership)

> Why don't Box (or other owning types) need a lifetime parameter?

A lifetime parameter is a bound on the owned type which a borrowed type is 'pointing to' (as we see later in the workshop, the compiler doesn't really know about pointing to explicitly, it just knows about lifetimes and types and how to associate the two). For an owning type, the value we're 'pointing to' has a lifetime derived from the owner, that lifetime is already known by the compiler, and there is no further lifetime which can give the compiler any more information.

> Why is it an ownership graph and not a tree?

Because of multiple ownership. Types such as `Rc` mean that any value in the graph can have more than one owner, and thus incoming edges.

> What is at the roots of the ownership graph?

First of all, does the ownership graph have roots? Yes. It is a DAG because ownership is not reflexive and because ownership cycles are not possible in Rust (reference cycles are possible, but ownership cycles are not).

The roots are each stack frame (each run of a function) which owns it's local variables, and any data with the `'static` lifetime (although these are borrowed, they can still own objects. Effectively they are owned by a run of the entire program).

> How does ownership relate to mutability?

Very loosely.

Mutability is derived from uniqueness. If we have a unique reference to an object it can be mutated. Some kinds of ownership are unique (e.g., `Box`), however, there can exist borrowed references and so even a uniquely object may have multiple references. Ownership can also be mutable. So no kind of ownership directly gives uniqueness. On the other hand, we can have unique references without ownership in the form of `&mut` references.

## Exercise 2 (types and control flow)

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
```

We can replace the first few lines by using the `?` operator:

```rust
fn foo(input: Option<i32>) -> Option<i32> {
    let input = input?;
    if input < 0 {
        return None;
    }
    Some(input)
}
```

Lets refactor that:

```rust
fn foo(input: Option<i32>) -> Option<i32> {
    let input = input?;
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}
```

We can then more easily see that we could use `and_then` because we return `Some` only if `input` is `Some` and the value is `>= 0`:

```rust
fn foo(input: Option<i32>) -> Option<i32> {
    input.and_then(|i| {
        if i >= 0 {
            Some(i)
        } else {
            None
        }
    })
}
```

However, we can do even better than that with a little digging in the standard library docs:

```rust
fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|i| *i >= 0)
}
```

`filter` for an `Option` works like `filter` on a list or iterator, it filters each element using the supplied function. In the `Option` case there can only be zero or one elements and so the output must also have zero or one elements, i.e., be another `Option`.

```rust
fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    match foo(input) {
        Some(n) => Ok(n),
        None => Err(ErrNegative),
    }
}
```

can be better written as


```rust
fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    foo(input).ok_or(ErrNegative)
}
```

`ok_or` converts from an `Option` to a `Result`, we supply the default value rather than use `ok_or_else` here because `ErrNegative` is a very small object and is basically free to create.


## Exercise 3 (iteration)


```rust
fn main() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}
```

is lowered to

```rust
fn main() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    loop {
        let v = match iter.next() {
            Some(v) => v,
            None => break,
        };
        println!("{}", v);
    }
}
```

## Exercise 4 (error handling)

Version with `expect`s.

```rust
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
```

Version with error handling

```rust
enum ServerError {
    DiskError(DiskError),
    NetError(NetworkError),
}

impl From<DiskError> for ServerError {
    fn from(e: DiskError) -> ServerError {
        ServerError::DiskError(e)
    }
}

impl From<NetworkError> for ServerError {
    fn from(e: NetworkError) -> ServerError {
        ServerError::NetError(e)
    }
}

fn read_config() -> Result<ConfigFile, DiskError> {
    let file = read_file()?;

    write_to_log()?;
    Ok(file)
}

impl Server {
    fn startup(self) -> Result<ListeningServer, ServerError> {
        let config = read_config()?;
        let port = open_port()?;
        Ok(self.configure(config, port))
    }
}

fn main() {
    let server = Server::new();
    let server = match server.startup() {
        Ok(server) => server,
        Err(e) => {
            eprintln!("...", e);
            exit(1);
        }
    };

    while let Some(packet) = server.listen() {
        // ...
    }
}
```

We first of all define our error type - `ServerError` - and implement `From` for it from the two lower level error types. These `From` implementations give `Into` implementations via a blanket implementation and when using `?` the compiler will use any available `Into` implementations to convert the type.

In `read_config` we simply replace each `expect` with a `?` and turn the return type into a `Result`. Note that since we can only get `DiskError`s in `read_config` we can use that error type. This way of writing the function implies there is no way to recover from these errors here, in reality, we might attempt some basic error recovery.

In `Server::startup` we again replace `expect`s with `?`s; however, in this case we take advantage of the implicit conversions thanks to the `From` implementations to return a `ServerError`.

In `main` we have to 'bottom out' our error handling. (In recent versions of Rust you can return a `Result` from `main`, but that is usually only a good idea for prototype code). We do this with a simple `match`, exiting with error code `1` if there was an error.

In a more realistic program, I would make the `ServerError` type more abstract rather than just wrapping the underlying errors. `startup` and similar public functions would then be a boundary of the 'error module'. We might make more effort to recover, perhaps retrying `open_port` or using a default config file (if that is appropriate). If we could not recover we'd return our more abstract error type which is friendlier to clients of our library and doesn't include implementation details such as *how* we connect to the network.


## Exercise 5 (abstraction with traits)

There are lots of places you can find places for better abstraction, however, this is a pretty difficult exercise - without deeply understanding a system it is usually hard to refactor.

Two examples from my own work:

### Cargo src

[file controller](https://github.com/nrc/cargo-src/blob/master/src/file_controller/mod.rs)

The file controller is a bit of a ball of mud which could do with a lot of general refactoring. One of it's functions is to read files from the file system, and cache them and a processed version for later use. This is delegated to the rls-vfs crate (Rust Language Server virtual file system).

An opportunity for abstraction here would be to have a `Cache` trait which abstracted over the above functionality. An advantage would be that we could unit test the rest of the file controller module, without having to setup and tear-down state in a real file system.


### GraphQL

[parser](https://github.com/nrc/graphql/blob/0a577fc765d450b5ddf8a82f5dfa401e8c320392/graphql/src/parser/parse_base.rs)

In this parser there is a concrete representation of a source of tokens (which is basically a `Vec<Token>`). We could use a trait for that. We would only need `next_tok` and `peek_tok` methods and the rest of the functionality could be provided as default methods. The advantage of this is that we could use a streaming iterator of tokens if we wanted to handle partial documents (e.g., as they arrive over a network), or implement all kinds of optimised versions where we don't want to keep all tokens around for all time.
