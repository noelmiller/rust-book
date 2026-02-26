fn main() {
    let s = String::from("hello"); // creates a string from a string literal

    let mut s = String::from("hello");

    s.push_str(", world"); // push_str() appends a literal to a String

    println!("{s}");

    // integers are known at a fixed size and are pushed onto the stack
    let x = 5;
    let y = x;

    // a string is made up of 3 parts: a pointer in memory that holds the contents,
    // a length, and a capacity. This group of data is stored on the stack.
    // since s2 points to the same data pointer as s1, this will create a double free error
    // because when both go out of scope, they will try to free the same piece of memory.
    // Freeing memory twice can create emory corruption, leading to a vulnerablility.
    // To protect us, rust invalidates s1 when s2 is created. If we try to reference s1,
    // we will get an error for an invalidated reference. This operation is considered a
    // move.
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // will cause an error
    println!("{s2}");

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!"); // ahoy world!

    // deep copy the heap data of the string, not just the stack data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    ownership_and_functions();
    return_value_and_scope();

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

// the primary issue with this function is we have to return String in order for it to be used again.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
fn scope() {
    // s is not valid here, since it is not yet declared
    let s = "hello"; // s is valid from this point forward
    // do stuff with s
} // the scope is now over, and s is no longer valid

fn scope_String() {
    let s = String::from("hello");

    // do stuff with s
} // this scope is now over, and s is no longer valid

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here
    // println!("testing {s}"); // this will not work.

    let x = 5; // x comes into scope
    println!("testing scope {x}"); // because of the Copy trait, this does still work

    makes_copy(x); // because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn return_value_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves it's return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back
    // which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
// s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return
    // value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
