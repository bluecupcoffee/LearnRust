use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
pub struct Tree {
    pub pwd: PathBuf,
    files: bool,
    sub_dirs: bool,
    recursion: bool,
}

impl Tree {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Tree, &'static str> {
        args.next();

        let pwd = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a starting dir"),
        };

        let opts = match args.next() {
            Some(o) => {
                if o[0..1].contains("-") {
                    o
                } else {
                    return Err("Improper flag arg");
                }
            }
            None => String::from(""),
        };

        Ok(Tree {
            pwd: PathBuf::from(pwd),
            files: opts.contains("f"),
            sub_dirs: opts.contains("d"),
            recursion: opts.contains("r"),
        })
    }

    pub fn discover_pwd(pwd: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let pwd_results = fs::read_dir(&pwd)?;
        let paths: Vec<PathBuf> = pwd_results
            .filter(|e| match e {
                Ok(_e) => true,
                Err(_err) => false,
            })
            .map(|p| p.unwrap().path())
            .collect();
        Ok(paths)
    }

    pub fn sort_paths(&self, paths: &mut Vec<PathBuf>) -> [Vec<PathBuf>; 2] {
        let mut path_vec: [Vec<PathBuf>; 2] = [vec![], vec![]];

        paths.iter().for_each(|p| {
            if p.is_file() && self.files {
                path_vec[0].push(p.to_path_buf());
            } else if p.is_dir() && self.sub_dirs {
                path_vec[1].push(p.to_path_buf());
            }
        });
        path_vec
    }

    pub fn dfs_exploration(
        &self,
        paths_arr: [Vec<PathBuf>; 2],
        mut starting_dir: (PathBuf, bool),
    ) -> Vec<PathBuf> {
        let mut full_list: Vec<PathBuf> = Vec::new();
        let mut dirs_todo: Vec<PathBuf> = Vec::new();
        while starting_dir.1 {
            starting_dir.1 = false;
            let mut new_paths = Self::discover_pwd(starting_dir.0.clone()).unwrap();
            let mut new_sorted = self.sort_paths(&mut new_paths);
            full_list.append(&mut new_sorted[0]);
            dirs_todo.append(&mut new_sorted[1]);
            match dirs_todo.pop() {
                Some(d) => {
                    if self.recursion {
                        starting_dir.0 = d.clone();
                        starting_dir.1 = true;
                    }
                    full_list.push(d);
                }
                None => break,
            };
        }
        full_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_all_opts() {
        let mut test_args = vec![
            "garbage".to_string(),
            r"Z:\".to_string(),
            "-frd".to_string(),
        ];

        let dummy_tree = Tree {
            pwd: PathBuf::from(r"Z:\"),
            files: true,
            sub_dirs: true,
            recursion: true,
            dfs: Vec::new(),
            visited_paths: Vec::new(),
        };

        let res = Tree::build(test_args.iter().map(|args| args.to_string()));
        let res = res.unwrap();
        assert_eq!(res.pwd, dummy_tree.pwd);
        assert_eq!(res.files, dummy_tree.files);
        assert_eq!(res.sub_dirs, dummy_tree.sub_dirs);
        assert_eq!(res.recursion, dummy_tree.recursion);
        assert_eq!(res.dfs.len(), dummy_tree.dfs.len());
        assert_eq!(res.visited_paths.len(), dummy_tree.visited_paths.len());
    }
}
