#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

impl Rectangle {
    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle {
            length: length,
            width: width,
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("Creating Rectangle: ");

    let rect = Rectangle {
        width: 30,
        length: 40,
    };

    // this is only possible by adding debug traits.
    println!("first rectangle is {:#?}", rect);
    println!("length of rectangle is {} cm", rect.length);
    println!("width of rectangle is {} cm", rect.width);
    println!("the area of rect is {} cm2", get_rectangle_area(&rect));

    // calling from function that bind to rectangle.
    println!("The area of rect is {} cm2", rect.area());

    // new way to create rectangle using struct function.
    let rect2 = Rectangle::new(50, 60);
    println!("The are of rect is {} cm2", rect2.area());

    // resuing exiting part of rect to create a new rectangle.
    let rect3 = Rectangle {
        length: 500,
        ..rect
    };

    println!("the length of rect3 is {} cm", rect3.length);
    println!(
        "the width of rect 3 is {} cm which is same as width of rect",
        rect3.width
    );
}

// this function is not bind to rectangle this is normal function.
fn get_rectangle_area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
