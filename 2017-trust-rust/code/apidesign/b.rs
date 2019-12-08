#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    body: String,
}

impl Book {
    fn new(title: String, author: String, body: String) -> Self {
    
        Book { title: title, author: author, body: body }
    
    }
}

fn write(b: &mut Book) {
    // write something, don't forget the errors :)
    b.body.push_str("This were the story of Rust, an magnificant language!");
}

fn edit(b: &mut Book) {
    // a spelling mistake remains, for the proof reader to catch!
    b.body = "This is the story of Rust, a magnificant language!".to_string();
}

fn proofread(b: &mut Book) {
    b.body = "This is the story of Rust, a magnificent language!".to_string();
}

fn main() {
    
    let mut b = Book::new(
        "Rust in 24 hours!".to_string(),
        "Joe Hacker".to_string(),
        String::new(),
    );

    write(&mut b);
    // Only a single write allowed as per the API spec.
    // But the compiler won't catch it :(
    write(&mut b);

    edit(&mut b);
    proofread(&mut b);
    println!("{:?}", b);
}




