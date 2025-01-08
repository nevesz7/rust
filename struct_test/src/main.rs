struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle {
        width: 40,
        height: 40,
    };

    println!("The area of the rectangle is {}", area(&rectangle));
    println!("The area of the square is {}", area(&square));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}