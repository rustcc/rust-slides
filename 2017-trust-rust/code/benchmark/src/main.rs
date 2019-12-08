#![feature(test)]

extern crate test;
extern crate rand;

use rand::Rng;

fn count_doubles(s: &str) -> u64 {
    let mut total = 0;
    
    for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
   }
   total
}

fn rand_string(n: usize) -> String {
    let mut rng = rand::thread_rng();
    let s:String = rng.gen_ascii_chars().take(n).collect();
    s
}

fn main() {
    
    let s = rand_string(1000000);
    println!("{}", count_doubles(&s));
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn bench_count_doubles(b: &mut Bencher) {
        let s = rand_string(1000_000);
        b.iter(|| count_doubles(&s));
    }
}


