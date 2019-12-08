
# Struct implementation

```rust,editable
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }
}

fn main() {
    let p = Person::new("John","Smith");
    println!("This is {} {}", p.first_name,p.last_name);
}
```

(`String` are objects, `&str` are references to char arrays)

