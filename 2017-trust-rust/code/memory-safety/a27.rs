fn vector_sum(v: Vec<i32>) -> i32 {
    //assume v is always a 3 elemnt vector
    v[0] + v[1] + v[2]
}
fn vector_product(v: Vec<i32>) -> i32 {
    //assume v is always a 3 element vector 
    v[0] * v[1] * v[2]
}
fn main() {
    let v = vec![1,2,3];
    
    let s = vector_sum(v);
    let p = vector_product(v);

    println!("{}",p);
    
}
