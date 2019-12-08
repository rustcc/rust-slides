
fn fun(a: &[u8]) {

    println!("a[3] = {}", a[3]);

}

fn main() {
    let  p = [10,20,30,40,50,60];
    fun(& p[1..4]);

}

