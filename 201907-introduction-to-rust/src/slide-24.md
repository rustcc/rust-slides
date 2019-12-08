
# More pattern matching

&nbsp;

```rust,editable
fn main() {
    let n = 0;
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };

    println!("{} is {}", n, text);
}
```

