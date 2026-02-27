#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // 2 different ways to print debug information to stdout
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    // print debug output to stderr
    // we need to use a reference because
    // debug will take ownership if we don't
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
