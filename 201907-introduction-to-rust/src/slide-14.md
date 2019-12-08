
# Function declaration

Parameters and return types must be explicit

```rust,editable
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn main() {
    for i in 0..5 {
        let even_odd = if is_even(i) {"even"} else {"odd"};
        println!("{} is {}", i, even_odd);
    }
}
```

