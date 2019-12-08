
# Tuples

&nbsp;

```rust,editable
fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn main() {
    let t = add_mul(2.0, 10.0);

    println!("tuple is {:?}", t);

    println!("add {} mul {}", t.0, t.1);

    let (add, mul) = t;
    println!("add {} mul {}", add, mul);
}
```

