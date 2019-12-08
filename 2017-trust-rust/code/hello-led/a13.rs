
fn main() {
    let v1 = vec![10, 20, 30, 40, 50];
    
    let v2:Vec<&i32> = v1.iter().skip(1).collect();

    println!("{:?}", v2);

}
