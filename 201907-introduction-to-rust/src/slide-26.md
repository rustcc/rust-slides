
# Structs

&nbsp;

```rust,editable
struct Person {
    first_name: String,
    last_name: String
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("This is {} {}", p.first_name, p.last_name);
}
```

