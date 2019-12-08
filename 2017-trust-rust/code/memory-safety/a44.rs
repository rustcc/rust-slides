fn main() {
    let a: *mut u32 = 0 as *mut u32;

    unsafe{
        *a = 0;
    }
}
