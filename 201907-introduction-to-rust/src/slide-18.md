
# Mutable function parameters

&nbsp;

```rust,editable
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut result = 0.0;
    modifies(&mut result);
    println!("result is {}", result);
}
```

