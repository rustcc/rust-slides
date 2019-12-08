
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> &i32 {
    
    &v1[0]

}

fn main() {

    let v1 = vec![1, 2, 3];
    let p:&i32;
    {
        let v2 = vec![4, 5, 6];
        p = foo(&v1, &v2);
        // How does the compiler know, just by looking at
        // the signature of "foo", that the reference returned
        // by "foo" will live as long as "p"?
    }

}
