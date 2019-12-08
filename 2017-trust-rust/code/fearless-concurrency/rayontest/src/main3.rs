extern crate rayon;
use rayon::prelude::*;
use std::cell::Cell;

fn main() {
    let n = Cell::new(0);
    let x:Vec<u64> = (0..100000000).collect();
    println!("collected!");
    
    let y:Vec<u64> = x.par_iter()
                     .map(|a|{
                        // count even numbers
                        if a % 2 == 0 { n.set(n.get()+1); }
                        a*a})
                      .collect();
    println!("{:?}", n);                  
}
