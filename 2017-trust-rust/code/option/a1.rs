
fn search(dict: &[(i32, i32)], key: i32) -> i32 {

    for &(k, v) in dict {
        if k == key {
            return v;
        }
    }
    
    return -1;
}

fn main() {
    let a = [(1, 20), (3, 40), (5, 80)];
     
    let r = search(&a[..], 8);

    println!("{}", r);
}
