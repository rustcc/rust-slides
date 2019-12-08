
struct Foo(i32);

fn main() {
    let x = Foo(10);

    let Foo(y) = x;
    let z = x.0;

    println!("{}, {}", y, z);
}
