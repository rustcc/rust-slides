
extern "C" {
    fn consume_array(
        offset: *const i32,
        length: usize,
    );
}

#[no_mangle]
extern "C" fn array_example() {
    let array = vec![1, 2, 3];
    unsafe { consume_array(array.as_ptr(), array.len()); }
}

#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
