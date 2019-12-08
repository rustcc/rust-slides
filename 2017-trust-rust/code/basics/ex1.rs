// Exercise!

// Implement a function which checks whether "n" 
// is prime. 0 and 1 are not prime.

// Hint: Better to avoid an explicit loop and use
// iterator functions. Two functions "take_while"
// and "all" will do the job!

// Examples:

// (2..100).take_while(|x| x < 15) will generate
// a sequence 2,3,4, ... 14. take_while will keep on
// accepting elements as long as the closure returns 
// true.

// (2..100).all(|x| (x % 2) == 0) evaluates to false.
// "all" returns true only if the closure returns true
// for all items of the sequence.

// You can check whether "n" is a prime by dividing 
// n by each each "i" where "i" ranges from 2 to n-1.
// Actually checking till i == sqrt(n) or (i*i) == n
// is sufficient.


fn is_prime(n: i32) -> bool {


}


fn main() {


}


