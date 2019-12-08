
fn fun(v: Vec<i32>) {
    println!("inside fun:{:?}", v);
}

fn main() {
    let v = vec![10, 20, 30];
    
    // vector is moved!
    fun(v);

    println!("{:?}", v);

}


