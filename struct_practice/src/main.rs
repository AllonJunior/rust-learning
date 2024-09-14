struct User{
    active: bool,
    username: String,
    address: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    /*
    let user1 = User{
        active: true,
        username: String::from("value"),
        address: String::from("Beijing"),
        email: String::from("user@example.com"),
        sign_in_count: 0,
    };
    println!("The user email is: {:?}", user1.email);
    */
    let width = 32;
    let height = 36;
    let area = rectangle_area(width, height);
    println!("The rectangle area is: {area}");

    // tuple rectangle area
    println!("The area is: {}", rect_area_tup((width, height)));

    // struct rectangle area
    let rectangle = Rectangle{
        width: 20,
        height: 30
    };
    println!("The struc rectangle area is: {}", rect_area_strut(&rectangle));

    // Print a rectangle directly after the Rectangle struct derived Debug
    println!("The rectangle is: {:#?}", rectangle);

    // output debug information by dbg!
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(20 * scale),
        height: 30
    };
    dbg!(&rect);
    
}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}
fn rect_area_tup(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}
fn rect_area_strut(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}
