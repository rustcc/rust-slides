
fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        assert_eq!(2, add_two_numbers(1, 1));
    }

    #[test]
    fn test2() {
        assert_eq!(0, add_two_numbers(-1, 1));
    }
}

