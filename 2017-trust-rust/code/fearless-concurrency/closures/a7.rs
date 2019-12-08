fn main() {
    
    let v = vec![1, 2, 3];

    let f = move |x| v[0] + x ;

    println!("{}", f(10));
}
