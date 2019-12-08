
fn main() {
    let  mut v = vec![1, 2, 3, 4];
    let mut v2 = vec![10, 20, 30];

    let mut f = |x| { v[0] += 1; x + v2[0] };

    println!("{}", f(10));

    let p = &mut v2;

    let q = &v;
}
