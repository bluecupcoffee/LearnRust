fn main() {
    println!("Hello, world!");
    // without struct
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels", two_var_area(width1, height1));
    // with struct
    let rect = Rectangle {
        width: 100,
        height: 100
    };
    let rect2 = Rectangle {
        width: 200,
        height: 200
    };
    let rect3 = Rectangle {
        width: 50,
        height: 50
    };
    let square = Rectangle::square(30);

    println!("The area of a Rectangle struct is {} square pixels", rect_area(&rect));
    println!("rect is \n{:#?}", rect);
    // with tuple
    let rect_tuple: (u32, u32) = (30, 50);
    println!("The area of the struct rectangle is {} square pixels", struct_area(rect_tuple));
    // with struct method
    println!("The area of the struct rectangle is {} square pixels (but we're \
    using a method instead of a function this time", rect.area());
    // can fit usage
    println!("rect can fit rect2? {}", rect.can_hold(&rect2));
    println!("rect can fit rect3? {}", rect.can_hold(&rect3));
    println!("Square area: {}", square.area());
    println!("rect can fit square? {}", rect.can_hold(&square));

}

fn two_var_area(width: u32, height: u32) -> u32 {
    width * height
}

fn struct_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}