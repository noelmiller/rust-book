fn main() {
    println!("Hello, world!");

    another_function();
    param_function(5);
    multiple_param_function(5, 'h');

    let x = return_function();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function")
}

fn param_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_param_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn return_function() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
