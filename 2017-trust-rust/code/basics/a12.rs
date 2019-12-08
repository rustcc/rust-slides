
fn fun(a: &[u8]) {

    println!("{:?}", a);

}

fn main() {
    let  p = [1,2,3];
    fun(&p[..]);
}

