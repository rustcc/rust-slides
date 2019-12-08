fn main() {
    let mut cnt = 0usize;
    crossbeam_utils::thread::scope(|s| {
        let cnt = &mut cnt as *mut usize as usize;
        for _ in 0..4 {
            s.spawn(move |_| {
                for _ in 0..1000 {
                    unsafe { *(cnt as *mut usize) += 1 };
                }
            });
        }
    }).unwrap();
    println!("{}", cnt);
}
