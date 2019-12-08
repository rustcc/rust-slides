use std::fs::File;

fn use_file() {
    let f = File::open("link.txt").unwrap();
}

fn main() {
    loop {
        use_file();
    }
}


