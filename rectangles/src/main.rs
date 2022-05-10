#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn build_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rec4 = Rectangle::build_square(12);

    println!("The area is {} square pixels.", rec1.area());
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec4 hold rec3? {}", rec4.can_hold(&rec3));
    println!("Rec1 is {:?}", rec1);
}
