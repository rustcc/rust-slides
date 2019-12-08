
# Functional iteration

&nbsp;

```rust,editable
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn main() {
    let sum: i32 =
        (0..5)                   // this is an iterator
        .filter(|i| is_even(*i)) // filter with a closure
        .sum();                  // consume the iterator
        
    println!("sum of even numbers is {}", sum);
}
```

