
# There can be only one owner

```rust,editable
#[derive(Debug)]
struct Person { name: String }

impl Person {
    fn new(name: &str) -> Person {
        Person { name: name.to_string() }
    }
}

fn take_ownership(p: Person) {
    println!("{} is mine", p.name);
}

fn borrow_it(p: &Person) {
    println!("I'm giving {} back to you!", p.name);
}

fn main() {
    let p = Person::new("John");
    println!("{:?}", p);
    
    // let x = p;  // moving p will break the code below
    // println!("{:?}", x);
    
    borrow_it(&p);
    println!("{:?}", p);
    
    take_ownership(p);    
    // println!("{:?}", p); // will fail
}
```

