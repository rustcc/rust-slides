
# Destructuring assigment

&nbsp;

```rust,editable
fn main() {
    let v = vec![10, 20, 30];    
    let idx = 0;
    
    if let Some(value) = v.get(idx) {
        println!("Value is {}", value);
    }
}
```

