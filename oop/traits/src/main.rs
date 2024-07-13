fn main() {
    println!("Hello, world!");
    let screen = Screen {
        components: vec! [
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("ok"),
                    String::from("no"),
                    String::from("alright")
                ]
            }),
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("button")
            })
        ]
    };

    screen.run();
}


pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("this doesn't actually draw anything but this is for a \"button\"");
    }
}

pub struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("this doens't actually draw anything but this is for a \'SelectBox\'");
    }
}

// different from defining a struct with a
// generic type parameter with trait bounds

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// would restrict you to generic types with a homogenous collection
// for the vec

// trait objects use dynamic dispatch
// generics are monomorphized and use static dispatch
// dynamic dispatch has a small performance penalty
// you get some flexibility with trait objects so its
// worth considering if you can afford the performance overhead
// at runtime