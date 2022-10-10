fn main() {
    let user1 = User {
        active: true,
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    /* Creating instances from other instances with Struct update syntax */
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax .. specifies that the remaining fields
    // not explicitly seet should have the same value as the fields in the given
    // instance
    let user4 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // Using tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Using the unit struct
    let subject = AlwaysEqual;
}

/* Defining Structs */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// function to build a user and return the user
fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        email, // because the email field and email parameter have the same name
        // we only need to write email rather than email: email
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/* Using tuple struct without named field */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit like struct without any fields */

struct AlwaysEqual;
