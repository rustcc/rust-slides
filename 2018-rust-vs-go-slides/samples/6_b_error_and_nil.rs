struct Gopher {
    name: String
}

fn main() {
    
    match find_gopher() {
        Err(err) => println!("ERROR: {}", err),
        Ok(gopher) => println!("Gopher: {}", gopher.name)
    };
    
    let gopher = match maybe_find_gopher() {
        Some(gopher) => gopher,
        None => Gopher { name: String::from("Test Tube Gopher")}
    };
    
    println!("Gopher: {}", gopher.name);
    
    // run time panics
    // find_gopher().unwrap();
    // maybe_find_gopher().unwrap();
}

fn find_gopher() -> Result<Gopher, String> {
    Err(String::from("Could not find any gophers, must be hibernating"))
}

fn maybe_find_gopher() -> Option<Gopher> {
    None
} 

