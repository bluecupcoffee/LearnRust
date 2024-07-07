use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("Hello, world!\nBox: {b}");
    
    let vec2: Vec<Box<DirEntry>> = vec![
        Box::new(DirEntry {
            entry_path: String::from("/root/folder/ftest.txt"),
            children: None
        }),
        Box::new(DirEntry {
            entry_path: String::from("/root/folder/ftest2.txt"),
            children: None
        })
    ];

    let vec1: Vec<Box<DirEntry>> = vec![
        Box::new(DirEntry {
            entry_path: String::from("/root/folder"),
            children: Some(vec2),
        }),
        Box::new(DirEntry {
            entry_path: String::from("/root/test.txt"),
            children: None
        }),
        Box::new(DirEntry {
            entry_path: String::from("root/test2.txt"),
            children: None
        })
    ];
    
    
    let de1 = DirEntry {
        entry_path: String::from("/root"),
        children: Some(vec1)
    };

    println!("Hola!\n{de1:#?}");

    let list = List::Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
    println!("Recursive list:\n{list:#?}");


}
#[derive(Debug)]
struct DirEntry {
    entry_path: String,
    children: Option<Vec<Box<DirEntry>>>
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}
// fake dir
// root
// L test.txt
// L test2.txt
// L folder
//      L ftest.txt
//      L ftest2.txt