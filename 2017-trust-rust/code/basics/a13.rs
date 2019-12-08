
// check a13-0.rs also

fn fun(a: &[u8]) {

    println!("{:?}", a);

    
}

fn main() {
    let p = [10,20,30,40,50,60];
    fun(&p[1..4]);
}

