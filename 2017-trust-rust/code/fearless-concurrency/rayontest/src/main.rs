extern crate rayon;
use rayon::prelude::*;

fn main() {
    let x:Vec<u64> = (0..100_000_000).collect();
 
    println!("collected!");
 
    // Run "htop" to see all CPU cores being busy!   
    let y:Vec<u64> = x.par_iter()
                     .map(|a| a * a)
                     .collect();

    println!("{:?}", y);

}
