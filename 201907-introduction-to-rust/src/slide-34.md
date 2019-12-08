
# Extending existing types

```rust,editable
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("a four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("an eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let pi = 3.14;
    println!("Here is {}", answer.show());
    println!("Here is {}", pi.show());
}
```

Sweet, we've added new methods to `i32` and `f64`!

