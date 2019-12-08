
fn main() {
    let v1 = vec![10, 20, 30, 40, 50];
    
    let v2 = vec![60, 70, 80, 90, 100];

    // Improves readability for long chains

    let v3:Vec<&i32> = v1.iter()
                       .chain(v2.iter())
                       .collect();

    println!("{:?}", v3);

}
