
fn identity(x: i32) -> i32 {
    x
}

fn identity(x:f32) -> f32 {
    x
}

fn main() {
    let p = identity(10);
    println!("{}", p);
    let q = identity(1.2);
    println!("{}", q);
    
}
