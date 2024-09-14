#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated function without self
    fn sqaure(size: u32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 22,
        height: 23,
    };
    
    let rect1 = Rectangle{
        width: 10,
        height:20,
    };
    let rect2 = Rectangle{
        width: 20,
        height: 30,
    };

    let square = Rectangle::sqaure(3);

    println!("The area is: {:#?}", rect.area());
    println!("Can the rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can the rect hold rect2? {}", rect.can_hold(&rect2));
    println!("The square is: {:#?}", square);
}
