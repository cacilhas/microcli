use std::{env, path::{Path, PathBuf}};
use git2::Repository;

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = &args[1..];
    if params.is_empty() {
        panic!("no directory supplied");
    }
    let param_string = params.join(" ");
    let origin = Path::new(&param_string);
    let path = PathBuf::from(origin).canonicalize().expect("not a real path");
    print_branch(&path);
}

fn print_branch(p: &Path) {
    match Repository::open(p) {
        Ok(repo) =>
            match repo.head() {
                Ok(head) =>
                    match head.shorthand() {
                        Some(branch) => println!("{}", branch),
                        None               => println!("no branch found"),
                    },
                Err(_) => println!("no head"),
            },
        Err(_) => {
            match p.parent() {
                Some(parent) => print_branch(&parent),
                None => (),
            }
        },
    }
}
