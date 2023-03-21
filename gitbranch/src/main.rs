use std::{
    env,
    error,
    fmt,
    path::{Path, PathBuf},
};

use git2::Repository;
use ParamsError::*;


fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let params = &args[1..];
    if params.is_empty() {
        ParamsError::throw(NoDirectory)?;
    }
    let param_string = params.join(" ");
    let origin = Path::new(&param_string);
    let path = PathBuf::from(origin).canonicalize()?;
    print_branch(&path);
    Ok(())
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


#[derive(Debug)]
enum ParamsError {
    NoDirectory,
}

impl fmt::Display for ParamsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no directory supplied")
    }
}

impl error::Error for ParamsError {
}

impl ParamsError {
    fn throw(err: ParamsError) -> Result<(), ParamsError> {
        Err(err)
    }
}
