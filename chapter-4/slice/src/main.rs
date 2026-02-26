fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content
    // that we could meaningfully use with the value 5, so the word is
    // now totally invalid

    // slices solve this problem
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // if you start from first byte, you can just do ..value
    let slice = &s[..2];

    // if you want up to the last byte you can just do value..
    let slice = &s[3..];

    // if you want to slice the entire string
    let slice = &s[..];

    let my_string = String::from("hello world");

    // first_word_slice works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // works on references too, which are equivalent to whole slices of Strings
    let word = first_word_slice(&my_string);

    // this will also work with string literals
    let my_string_literal = "hello world";

    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    let word = first_word_slice(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// it is better to take a reference to a string
// because we can take a string slice or a whole
// string.
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
