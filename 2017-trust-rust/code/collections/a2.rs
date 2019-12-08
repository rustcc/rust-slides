
use std::collections::HashMap;

fn main() {
    let mut price_list = HashMap::new();
    
    price_list.insert("mango", 100);
    price_list.insert("apple", 110);
    price_list.insert("orange", 90);

    let t = price_list.contains_key("orange");
    println!("contains orange = {}", t);

    let t = price_list.get("orange");
    println!("value associated with orange = {:?}", t);
 
    for (fruit, price) in &price_list {
        println!("{:?}, {:?}", fruit, price);
    }   
}

