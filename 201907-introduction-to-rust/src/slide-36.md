
# Box: dynamic allocation

```rust,editable
#[derive(Debug)]
struct Node {
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(s: &str) -> Node {
        Node{value: s.to_string(), left: None, right: None}
    }

    fn set_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    fn set_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }
}


fn main() {
    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));

    println!("{:#?}", root);
}
```

