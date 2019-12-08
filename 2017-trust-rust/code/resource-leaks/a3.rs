
use std::fs::File;

fn use_file() {
    let f = File::open("link.txt").unwrap();
}

fn main() {
    use_file();
    println!("good bye!");
}

// verify "close" is being called using "strace"

