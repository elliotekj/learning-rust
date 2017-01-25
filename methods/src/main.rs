#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.length > target.length
    }

    // This is known as an associated function: it's part of the Rectangle strtuct's
    // implementation but doesn't take `&self` as a param. Often used for constructors.
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 50, length: 30 };
    let rect2 = Rectangle { width: 40, length: 60};
    let rect3 = Rectangle { width: 2, length: 2};

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(47);
    println!("The area of the square is {} square pixels.", sq.area());
}
