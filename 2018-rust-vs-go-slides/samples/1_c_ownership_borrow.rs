fn main() {
    // _Stack_ allocated integer
    let anns_cheap_integer = 5;

    // *Copy* 'anns_cheap_integer' into 'bobs_integer' - no resources are 'moved'.
    // Primitive types are copy by default because their size is
    // known at compile time and allocated cheaply on stack.
    let bobs_integer = anns_cheap_integer;

    // Both values can be independently used
    println!("ann has {}, and bob has {}", anns_cheap_integer, bobs_integer);

    // 'anns_expensive_vector' points to a _heap_ allocated, dynamically growable list
    // underlying size is not known at compile time
    let anns_expensive_vector = vec![1, 2, 3];

    // anns_expensive_vector' is *Moved* into 'bobs_vector'.
    // Ownership is transferred without any underlying data being copied.
    // 'bobs_vector' now owns the underlying data and 'anns_expensive_vector' is invalidated.
    let mut bobs_vector = anns_expensive_vector;
    //println!("ann's vector contains: {:?}", anns_expensive_vector); // ERROR

    // ownership can also be moved into functions
    give_away_vector(bobs_vector);
    //println!("bob's vector contains: {:?}", bobs_vector); // ERROR
    
    // lend the data without losing ownership
    //borrow_vector(&bobs_vector);
    //println!("bob's vector contains: {:?}", bobs_vector);

    // lend the data mutably without losing ownership
    //borrow_and_change_vector(&mut bobs_vector);
    //println!("bob's vector mutated: {:?}", bobs_vector)

} // everything is dropped at end of this scope

pub fn give_away_vector(vector: Vec<isize>) {
    println!("this function owns bob's vector {:?}", vector);
} // reference to vector is dropped at end of function scope and the memory freed
// not by a garbage collector, but by a compiler generated drop/dealloc call

pub fn borrow_vector(vector: &Vec<isize>) {
    println!("First of bob's vector is: {}", vector[0]);
}

pub fn borrow_and_change_vector(xs: &mut Vec<isize>) {
    xs.pop();
    xs.push(42);
}
