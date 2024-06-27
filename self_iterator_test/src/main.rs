use self_iterator_test::Tree;

fn main() {
    let tree = Tree::build(String::from("Z:\\"));
    match tree.discover_pwd() {
        Ok(r) => println!("OK!"),
        Err(e) => eprintln!("{e}"),
    }
}
