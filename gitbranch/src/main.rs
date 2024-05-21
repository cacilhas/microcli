//! It simply shows current branch name - or nothing if it isn’t a git repo.
//!
//! # Build and install
//!
//! ```sh
//! cargo install gitbranch
//! ```
//!
//! # Use
//!
//! ```sh
//! gitbranch [<directory>]
//! ```
//!
//! Directory defaults to current working directory.

use git2::Repository;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let params = &args[1..];
    let param_string = if params.is_empty() {
        env!["PWD"].to_owned()
    } else {
        params.join(" ")
    };
    let origin = Path::new(&param_string);
    let path = PathBuf::from(origin).canonicalize()?;
    print_branch(&path);
    Ok(())
}

fn print_branch(p: &Path) {
    match Repository::open(p) {
        Ok(repo) => match repo.head() {
            Ok(head) => match head.shorthand() {
                Some(branch) => println!("{}", branch),
                None => println!("HEAD"),
            },
            Err(_) => eprintln!("no head"),
        },
        Err(_) => match p.parent() {
            Some(parent) => print_branch(parent),
            None => (),
        },
    }
}
