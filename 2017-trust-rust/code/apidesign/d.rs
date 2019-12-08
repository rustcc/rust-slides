#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    body: String,
}

struct WritingCompletedBook {
    title: String,
    author: String,
    body: String,
}

struct EditingCompletedBook {
    title: String,
    author: String,
    body: String,
}

struct ProofReadingCompletedBook {
    title: String,
    author: String,
    body: String,
}
    
impl Book {
    fn new(title: String, author: String, body: String) -> Self {
    
        Book { title: title, author: author, body: body }
    
    }
}

fn write(b: Book) -> WritingCompletedBook {
    WritingCompletedBook {
        title: b.title,
        author: b.author,
        body: "This were the story of Rust, an magnificant language!".to_string(),
    } 
}

fn edit(b: WritingCompletedBook) -> EditingCompletedBook {
    EditingCompletedBook {
        title: b.title,
        author: b.author,
        body: "This is the story of Rust, a magnificant language!".to_string(),
    }
}

fn proofread(b: EditingCompletedBook) -> ProofReadingCompletedBook {
    ProofReadingCompletedBook {
        title: b.title,
        author: b.author,
        body: "This is the story of Rust, a magnificent language!".to_string(),
    }
}

fn main() {
    
    let  b = Book::new(
        "Rust in 24 hours!".to_string(),
        "Joe Hacker".to_string(),
        String::new(),
    );

    write(b);
    write(b);
}




