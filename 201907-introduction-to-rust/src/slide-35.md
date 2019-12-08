
# Adding type constraints

```rust,editable
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("a four-byte signed {}", self)
    }
}

impl<T> Show for Option<T> where T: Show {
    fn show(&self) -> String {
        match self {
            Some(v) => v.show(),
            None => format!("nothing"),
        }
    }
}

fn main() {
    let answer = Some(42);
    let void: Option<i32> = None;
    println!("Here is {}", answer.show());
    println!("Here is {}", void.show());
}
```

