fn main() {
    println!("Hello, world!");
    let c = CustomSmartPointer {
        data: String::from("haha hee hee ho ho hu hu")
    };

    let mut d = CustomSmartPointer {
        data: String::from("bleh")
    };
    println!("Created smart pointers");

    let e = &d;
    drop(e);
    let mut f = &mut d;
    f.data = String::from("ouch");
    println!("after drop statement");
    drop(f);
    println!("dropped reference to d");
    drop(d);
    println!("dropped d");

}

struct CustomSmartPointer {
    pub data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}