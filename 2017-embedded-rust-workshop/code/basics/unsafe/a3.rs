
fn main() {

    unsafe {
        let p1 = 0 as *mut i32;

        *p1 = 100; // no compile time error. But program crashes at run time!
    }

    // similar to writing, in C:
    // int *p; p = 0;
    // *p = 100;
}
