
# Pattern matching

&nbsp;

```rust,editable
fn main() {
    let v = vec![10, 20, 30]; // initialization macro    
    let idx = 0;
    
    match v.get(idx) {
        Some(value) => println!("Value is {}", value),
        None => println!("No value..."),
    }
}
```

