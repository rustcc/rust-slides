
struct EmailID(String);

struct PostalAddress {
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,
}

// A contact needs to have a name, and
// an email id or postal address.

// This is too weak: you can create a Contact
// with no email, no postal address.

struct Contact {
    name: String,
    email: Option<EmailID>,
    postal_address: Option<PostalAddress>,
}

fn main() {

}


