fn main() {
    // basic if else
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // checks if a number is there
    if number != 0 {
        println!("number was something other than zero");
    }

    // multiple ifs
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // you can assign a variable based on a condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    // infinite loop, can only be broken out of using control+c
    //loop {
    //    println!("again!")
    //}

    // loop using a break statement
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // using label to allow for loop control
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("reamining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!");

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for loop version of the while loop above. It is more efficient
    for number in { 1..4 }.rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
