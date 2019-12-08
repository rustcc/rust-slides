
fn foo<'a, 'b>(v1: &'a Vec<i32>, v2: &'b Vec<i32>) -> &'a i32 {
    
    &v1[0]

}

fn main() {

    let v1 = vec![1, 2, 3];
    let p:&i32;
    {
        let v2 = vec![4, 5, 6];
        p = foo(&v1, &v2);
    }

}
