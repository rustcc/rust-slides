
fn main() {
    let  mut v = vec![1, 2, 3, 4];

    let mut f = |x| { v[0] += 1; x + v[0] };

    println!("{}", f(10));


}
