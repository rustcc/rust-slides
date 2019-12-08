
fn foo() -> &Vec<i32> {
    let v = vec![1, 2, 3];
    &v // Will this compile? 
}

fn main() {
    let p = foo();
}
