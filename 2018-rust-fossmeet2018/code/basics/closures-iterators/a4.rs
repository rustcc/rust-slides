
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let v2:Vec<i32> = v.iter()
                      .filter(|x| *x % 2 == 0)
                      .map(|x| x * x).collect();

    println!("{:?}", v2);
}
