fn main() {

    // explicit type annotation
    let like_a_butterfly: f64 = 21.0;

    // type inference
    let int_eresting = 8;

    // opt in mutability
    let mut climate_change = false;
    climate_change = true;

    // variable shadowing
    let climate_change = 2018;
    
    print_things(like_a_butterfly, int_eresting, climate_change);
}

fn print_things(c: f64, a: usize, t: isize) {
    print!("{}-{}-{}", c, a, t);
}
