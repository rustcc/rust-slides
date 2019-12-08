
fn main() {
    let p1 = 0 as *mut i32;

    *p1 = 100;

    // similar to writing, in C:
    // int *p; p = 0;
    // *p = 100;
}
