
use std::collections::HashMap;

fn main() {
    let mut price_list: HashMap<&str, i32> = 
        [("mango", 100),
         ("apple", 110),
         ("orange", 90),].iter().cloned().collect();


    println!("{:?}", price_list);
}

