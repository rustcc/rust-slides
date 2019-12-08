#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

fn main() {
    let x = Maybe::Just(10);
    let y = Maybe::Just("hello");
    println!("{:?}, {:?}", x, y);
}
