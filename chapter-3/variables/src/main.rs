// Constants are always immutable, they can be defined in any scope (even globally as seen here),
// and they must be annotated with a type. They are most useful for values that the entire program
// needs to have access to.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let x = 5;
    // example of shadowing a variable in an outer scope
    let x = x + 1;
    {
        let mut y = 5; // mut allows the variable to change. Without mut, the variable cannot change
        println!("The value of y is: {}", y);
        y = 6;
        println!("The value of y is: {}", y);
        // example of shadowing a variable for an inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    println!(
        "The value of 3 hours in seconds is: {}",
        THREE_HOURS_IN_SECONDS
    );

    // shadowing is the most useful in the case of reusing the same variable in a case like this.
    // If we use a mut, we will get a compile time error because you can't mutate from one
    // type to another without redeclaring the variable.
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The amount of spaces is: {}", spaces);
}
