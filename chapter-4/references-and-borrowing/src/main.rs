fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1} is {len}.");

    let mut s = String::from("hello");

    // you cannot borrow a mutable reference more than one time.
    // an additional r2 here set to the same mutable reference
    // would cause an error. You could have it be in a different scope
    // which can be denoted by curly brackets.
    let r1 = &mut s;

    println!("{r1}");

    change(&mut s);
}

// this is an example of borrowing a reference.
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope. But because s does not have ownership of
// what it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
