
# If as an expression

&nbsp;

```rust,editable
fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} is {}", i, even_odd);
    }
}
```

