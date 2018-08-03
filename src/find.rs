use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::PathBuf;

fn find(query: &str, start: &OsStr) -> io::Result<Vec<PathBuf>> {
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
                if query.is_empty() || query == name {
                    result.push(path.clone());
                }
            }
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let query = args.next().unwrap_or(String::new());
    let start = args.next().map(OsString::from).unwrap_or(OsString::from("."));

    for path in find(&query, &start)? {
        if let Some(p) = path.to_str() {
            println!("{}", p);
        }
    }
    Ok(())
}
