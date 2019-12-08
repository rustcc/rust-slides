
# A generic sorted tree

```rust,editable
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(v: T) -> Node<T> {
        Node{value: v, left: None, right: None}
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }
    
    fn insert(&mut self, data: T) {
        if data < self.value {       // <-- Ord is used here
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
}

fn main() {
    let mut root = Node::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("four".to_string());

    println!("{:#?}", root);
}
```


