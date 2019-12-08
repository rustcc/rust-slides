
fn search(dict: &[(i32, i32)], key: i32) -> Option<i32> {

    for &(k, v) in dict {
        if k == key {
            return Some(v);
        }
    }
    
    None
}

fn main() {
    let a = [(1, 20), (3, 40), (5, 80)];
     
    let r1 = search(&a[..], 5);

    let r2 = r1.map(|x| 2 * x);

    match r2 {
        Some(x) => println!("x = {}", x),
        Nothing => println!("key not present"),
    }
}
