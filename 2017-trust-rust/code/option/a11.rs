
fn main() {
    let mut v:Vec<i32> = vec![];
    let m = v.pop(); 

    println!("m = {:?}", m);

    let t = m.unwrap();
}
