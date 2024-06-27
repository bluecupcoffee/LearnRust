use std::error::Error;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub struct Tree {
    pwd: String,
    files: Option<Vec<String>>,
    folders: Option<Vec<Tree>>,
}

struct Folder {
    folder_path: String,
    subtree: Tree,
}

impl Tree {
    pub fn build(pwd: String) -> Tree {
        Tree {
            pwd: pwd,
            files: None,
            folders: None,
        }
    }

    pub fn discover_pwd(&self) -> Result<bool, Box<dyn Error>> {
        let pwd_results = fs::read_dir(&self.pwd)?;
        let paths = pwd_results
            .filter(|e| match e {
                Ok(e) => true,
                Err(err) => false,
            })
            .map(|p| p.unwrap().path());
        paths.for_each(|p| println!("{p:?}"));
        Ok(true)
    }
}
