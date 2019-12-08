
fn foo() -> Vec<i32> {
    let v = vec![1, 2, 3];
    v // transfer ownership to caller
}

fn main() {
    let p = foo();
    println!("{:?}", p);
}
