fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false; // explicit type annotation

    // characters (note the single quotes compared to strings)
    let c = 'z';
    let z: char = 'Z'; // explicit type annotation
    let heart_eyed_cat = '😻'; // can use emojis too! Any valid unicode character works.

    // tuples
    // used to group multiple types together. Once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!(
        "The value of x is {}, the value of y is {}, and the value of z is {}",
        x, y, z
    );
    // you can also access the values by using dot notation
    println!(
        "The value of tup.0 is {}, the value of tup.1 is {}, the value of tup.2 is {}",
        tup.0, tup.1, tup.2
    );

    // arrays
    // arrays must have the same type as opposed to tuples
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // set the type and length of an arrary using type annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize an array with all the same value
    let c = [69; 5];

    // access elements of an array
    let sixty_nine = c[0];

    println!(
        "The value of c[0] is {} and the value of months[0] is {}",
        sixty_nine, months[0]
    );
}
