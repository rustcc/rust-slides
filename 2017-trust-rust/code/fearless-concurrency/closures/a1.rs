
fn main() {
    let  v = vec![1, 2, 3, 4];

    let  f = |x| { x + v[0] };

    println!("{}", f(10));

    println!("{:?}", v);

}
