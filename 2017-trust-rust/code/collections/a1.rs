
use std::collections::HashMap;

fn main() {
    let mut price_list = HashMap::new();
    
    price_list.insert("mango", 100);
    price_list.insert("apple", 110);
    price_list.insert("orange", 90);

    println!("{:?}", price_list);
}

