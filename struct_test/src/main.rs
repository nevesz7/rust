struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Square {
    rectangle: Rectangle,
}

impl Square {
    // Constructor to create a square
    fn new(side: u32) -> Square {
        Square {
            rectangle: Rectangle {
                width: side,
                height: side,
            },
        }
    }

    fn area(&self) -> u32 {
        self.rectangle.area()
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Square::new(40);

    println!("The area of the rectangle is {}", rectangle.area());
    println!("The area of the square is {}", square.area());
}
