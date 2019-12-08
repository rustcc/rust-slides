
fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];

    v1.iter()
    .zip(v2.iter())
    .for_each(|(x, y)| {
        println!("{:?}, {:?}", x, y);
    });

}
