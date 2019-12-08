#[derive(Debug)]

enum Maybe {
    Just(i32),
    Nothing,
}

use Maybe::*;

fn search(dict: &[(i32, i32)], key: i32) -> Maybe {

    for &(k, v) in dict {
        if k == key {
            return Just(v);
        }
    }
    
    Nothing
}

fn main() {
    let a = [(1, 20), (3, 40), (5, 80)];
     
    let r = search(&a[..], 5);

    match r {
        Just(x) => println!("x = {}", x),
        Nothing => println!("key not present"),
    }
}
