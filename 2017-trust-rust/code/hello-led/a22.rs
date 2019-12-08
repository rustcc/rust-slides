
// Another example of the iterator approach - 
// computing the edit distance.

fn main() {
    let s1 = "abcdef";
    let s2 = "apcder";

    let n = 
        s1.chars()
        .zip(s2.chars())
        .filter(|t| t.0 != t.1)
        .count();
    
    println!("{}", n);
}
