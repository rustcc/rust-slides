
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

enum ContactInfo {
    EmailOnly(EmailID),
    PostalOnly(PostalAddress),
    EmailAndPostal(EmailID, PostalAddress),
}

// This looks ok!
// The "illegal state" without both email-id and postal
// address simply can't be represented in the program.


struct Contact {
    name: String,
    contact_info: ContactInfo,
}


fn main() {

}


