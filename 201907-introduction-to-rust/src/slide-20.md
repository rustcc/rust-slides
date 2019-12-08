
# Vectors

&nbsp;

```rust,editable
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];           // will panic if out-of-range
    let maybe_first = v.get(0); // returns an Option

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
```

