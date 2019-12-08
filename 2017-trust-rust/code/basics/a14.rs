
fn main() {
    let mut a = [1,2,3];
    let mut b = a; 

    println!("{:?}, {:?}",a, b );

    b[0] = 10;

    println!("{:?},{:?}",a,b);

    // shows that b = a completely copies a to b
}
