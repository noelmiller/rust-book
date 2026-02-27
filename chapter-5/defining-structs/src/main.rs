fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someemail@email.com"),
        sign_in_count: 1,
    };

    println!("the email is: {}", user1.email);

    user1.email = String::from("anotheremail@email.com");

    println!("the changed email is: {}", user1.email);

    // using build_user to create a user
    let user2 = build_user(
        String::from("heyitsyaboyemail@email.com"),
        String::from("yaboy"),
    );

    // struct update syntax. This will inherit all of the values
    // of user1 except the email
    // be careful though! user1 is not usable now because its username got moved
    // and not copied. bools and u64s are able to be copied due to their nature.
    // Strings do not have that same property
    let user3 = User {
        email: String::from("duebroitsme@email.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct AlwaysEqual;

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // we can use shorthand because the parameters are the same as
        // what the struct needs
        username, // shorthand for username: username
        email,    // shorthand for email: email
        sign_in_count: 1,
    }
}
