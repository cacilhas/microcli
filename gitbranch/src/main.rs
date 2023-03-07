use std::{env, path::Path};
use git2::Repository;

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = &args[1..];
    if params.is_empty() {
        panic!("no directory supplied");
    }
    print_branch(&params.join(" "));
}

fn print_branch(p: &String) {
    if p == "" || p == "/" {
        return;
    }

    match Repository::open(p) {
        Ok(repo) => {
            match repo.head() {
                Ok(head) => {
                    let branch = head.shorthand().unwrap();
                    print!("{}", branch);
                },
                Err(_) => return,
            }
        },
        Err(_) => {
            match Path::new(p).parent() {
                Some(parent) => {
                    let s = parent.to_str().unwrap().to_string();
                    print_branch(&s);
                },
                None => return,
            }
        },
    }
}
