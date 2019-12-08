
#[macro_use] extern crate maplit;

fn main() {

    let fruits = hashmap! {
            "apple"  => 100,
            "orange" => 120,
            "mango"  => 130,
    };
        
    println!("{:?}", fruits);
}
