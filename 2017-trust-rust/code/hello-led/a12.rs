
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9, 10];

    // Don't worry about the &
    let v3:Vec<(&i32, &i32)> = v1.iter().zip(v2.iter()).collect();

    println!("{:?}", v3);

}
