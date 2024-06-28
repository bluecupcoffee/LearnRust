use self_iterator_test::Tree;
use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let tree = Tree::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error initializing tree: {err}");
        process::exit(1);
    });
    println!("{tree:?}");
    let mut path = Tree::discover_pwd(tree.pwd.clone()).unwrap_or_else(|err| {
        eprintln!("Error discovering pwd: {err}");
        process::exit(1);
    });

    let sorted_paths = tree.sort_paths(&mut path);

    let mut fl: Vec<PathBuf> = vec![tree.pwd.clone()];
    let mut res = tree.dfs_exploration(sorted_paths, (tree.pwd.clone(), true));
    fl.append(&mut res);
    println!("{fl:#?}");
}
