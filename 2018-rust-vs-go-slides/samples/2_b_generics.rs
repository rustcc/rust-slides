// Generic type T with trait bounds
// Ord trait means we can compare elements
// Clone trait is necessary because rust applies ownership rules even on elements
// of a container. Indexing an elem means to try to move it, and moving a partial
// part of the container would invalid the whole collection. So we must clone the elem
// we want to return.
fn max<T: Ord + Clone>(collection: Vec<T>) -> T {
    let mut max = &collection[0]; // still can panic
    
    for i in 0..collection.len() {
        if &collection[i] > max {
            max = &collection[i];
        }
    } 
    // no semicolon = expression
    max.clone()
}

fn main() {
    let some_ints = vec![3,4,1,7,8,324];
    let some_strings = vec!["cat", "dog", "horse", "sleeping gopher"];
    println!("{} {}", max(some_ints), max(some_strings));

    let some_nothing: Vec<u64> = vec![];

    // std lib max function for generic vector
    // returns an Option type expressing None if the list is empty
    println!("max = {:?}", some_nothing.iter().max());
}
