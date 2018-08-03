use std::env;
use std::io;
use std::collections::VecDeque;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;

fn find(query: &String, start: &OsString) -> io::Result<Vec<PathBuf>> {
    let start = PathBuf::from(start);
    let mut dirs = VecDeque::from(vec![start]);
    let mut result = Vec::new();

    while let Some(dir) = dirs.pop_front() {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                dirs.push_back(path.clone());
            }
            if let Some(name) = path.file_name() {
                if query.is_empty() || query.as_str() == name {
                    result.push(path.clone());
                }
            }
        }
    }
    Ok(result)
}

fn main() {
    let query = match env::args().nth(1) {
        Some(query) => query,
        None => String::new(),
    };
    let start = match env::args().nth(2) {
        Some(start) => OsString::from(start),
        None => OsString::from("."),
    };

    match find(&query, &start) {
        Ok(paths) => {
            for path in paths {
                if let Some(p) = path.to_str() {
                    println!("{}", p);
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        },
    };
}
