
use std::collections::HashSet;

fn main() {
    let mut fruits = HashSet::new();
    
    fruits.insert("apple");
    fruits.insert("orange");
    fruits.insert("banana");

    println!("{:?}", fruits);

    println!("contains orange? {}", fruits.contains("orange"));

    fruits.remove("orange");

    println!("contains orange? {}", fruits.contains("orange"));

    for fruit in &fruits {
        println!("{:?}", fruit);
    }

}
