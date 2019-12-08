
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref FRUITS: HashMap<&'static str, i32> =
        hashmap!{
            "apple" => 100,
            "orange" => 120,
            "mango" => 130,
        };
}

fn main() {

    println!("{:?}", FRUITS.get("orange"));

}
