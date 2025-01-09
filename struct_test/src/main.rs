struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
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

    println!("The area of the rectangle is {}", rectangle.area());
    println!("The area of the square is {}", square.area());
}
