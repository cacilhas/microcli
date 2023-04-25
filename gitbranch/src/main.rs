use std::{
    env,
    path::{Path, PathBuf},
};

use git2::Repository;


fn main() {
    let args: Vec<String> = env::args().collect();
    let params = &args[1..];
    let param_string = if params.is_empty() {
        env!["PWD"].to_owned()
    } else {
        params.join(" ")
    };
    let origin = Path::new(&param_string);
    let path = PathBuf::from(origin).canonicalize().unwrap();
    print_branch(&path);
}

fn print_branch(p: &Path) {
    match Repository::open(p) {
        Ok(repo) =>
            match repo.head() {
                Ok(head) =>
                    match head.shorthand() {
                        Some(branch) => println!("{branch}"),
                        None => eprintln!("no branch found"),
                    },
                Err(_) => eprintln!("no head"),
            },
        Err(_) => {
            match p.parent() {
                Some(parent) => print_branch(parent),
                None => (),
            }
        },
    }
}
