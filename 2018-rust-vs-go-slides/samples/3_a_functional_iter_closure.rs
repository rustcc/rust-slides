// import deps
use std::collections::BTreeSet;

// take a list of numbers, and filter, map, sort, unique, sum...
fn really_contrived_example(int_xs: Vec<isize>)-> isize {
    int_xs.iter()
        .filter(|&&x| x >= 10)
        .map(|x| x + 1)
        .collect::<BTreeSet<isize>>().iter()
        .take(3)
        .sum()
}

fn main() {
    let numbers = vec![1, 2, 10, 13, 16, 40, 50, 60];
    println!("{}", really_contrived_example(numbers)); // 42
}

