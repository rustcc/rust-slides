extern crate rayon;
use rayon::prelude::*;


fn main() {
    let mut n = 0;
    let x:Vec<u64> = (0..100000000).collect();
    println!("collected!");
    
    let y:Vec<u64> = x.par_iter()
                     .map(|a|{
                        // count even numbers
                        if a % 2 == 0 { n += 1; } 
                        a*a})
                      .collect();
    println!("{}", n);                  
}
