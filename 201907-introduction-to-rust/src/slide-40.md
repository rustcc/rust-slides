
# Reference counters

```rust,editable
use std::rc::Rc;

fn main() {
    let s = "hello dolly".to_string();
    
    let rs1 = Rc::new(s); // s moves to heap; ref count 1
    
    println!("{}", Rc::strong_count(&rs1));
    
    let rs2 = rs1.clone(); // ref count 2
    
    println!("{}", Rc::strong_count(&rs1));

    let rs3 = Rc::downgrade(&rs1); // weak reference, doesn't inc count

    println!("{:?}, {:?}, {:?}", rs1, rs2, rs3.upgrade());
    
    println!("Dropping strong references");
    drop(rs1);
    drop(rs2);
    println!("{:?}", rs3.upgrade());
}
```

