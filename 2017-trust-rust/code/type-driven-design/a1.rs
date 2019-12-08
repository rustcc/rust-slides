
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

// This is too strict: needs both email AND
// postal.

struct Contact {
    name: String,
    email: EmailID,
    postal_address: PostalAddress,
}

fn main() {

}


