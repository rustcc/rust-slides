
enum Role{
    Admin,
    Guest,
    Supervisor,
}

struct User {
    id: u32,
    role: Role,
}

fn main() {
    let u1 = User { id : 10, role : Role::Guest};
    
    match u1 {
        User {id : x, role : Role::Admin} => 
            println!("Admin with id {}", x),
        User {id : x, role : Role::Guest} => 
            println!("Guest with id {}", x),
        User {id : x, role : Role::Supervisor} => 
            println!("Supervisor with id {}", x),
    }
}
