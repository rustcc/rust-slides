
// check a13-1.rs also
fn fun(a: &mut [u8]) {

    println!("slice = {:?}", a);

    a[0] = 100;   
}

fn main() {
    let mut p = [10,20,30,40,50,60];
    fun(&mut p[1..4]);

    println!("after calling fun: {:?}", p);
}

