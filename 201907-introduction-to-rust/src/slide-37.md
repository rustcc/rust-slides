
# Automatic memory reclamation

`Box::new(node)` allocates on the heap and `node` is _moved_ inside the box. Ownership of the box can move, but you
can only get a reference to its content.

The memory is automatically freed when the box has no more owner (it is "dropped").

```rust,editable
struct DropTracer(i32);

impl Drop for DropTracer {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let a = DropTracer(0);
    println!("a contains {}", a.0);

    let mut b = Box::new(DropTracer(1));
    println!("b contains {}", b.0);
    
    println!("Replacing b");
    b = Box::new(DropTracer(2));
    println!("b contains {}", b.0);
   
    println!("Exiting");
}
```

