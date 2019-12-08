
struct Point(u32, u32);

fn main() {

    let p = Point(10, 20);

    let Point(x, y) = p;

    println!("{}, {}", p.0, p.1);
    println!("{}, {}", x, y);
}
