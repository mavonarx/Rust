#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(sidelength: u32) -> Self {
        Self {
            width: sidelength,
            height: sidelength,
            name: String::from("square"),
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
        name: "Matthias".to_string(),
    };

    let square = Rectangle::square(22);
    println!("{}{}{}", square.width, square.height, square.name);
    println!(
        "The area of the rectangle is {} square pixels. and its name is {}",
        rect1.area(),
        rect1.name
    );

    println!("rect1 is {:#?}", rect1);
}
