
fn sum_vector(v: Vec<i32>) -> i32 {
    v[0] + v[1] + v[2]
}

fn product_vector(v: Vec<i32>) -> i32 {
    v[0] * v[1] * v[2]
}

fn main() {
    let v = vec![1, 2, 3];

    let r1 = sum_vector(v);
    let r2 = product_vector(v);

    // does this compile?
}
