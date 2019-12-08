
fn identity_1(x: i32) -> i32 {
    x
}

fn identity_2(x:f32) -> f32 {
    x
}

fn main() {
    let p = identity_1(10);
    println!("{}", p);
    let q = identity_2(1.2);
    println!("{}", q);
    
}
