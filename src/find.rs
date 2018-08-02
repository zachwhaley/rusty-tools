use std::env;
use std::fs;
use std::io;
use std::collections::VecDeque;
use std::path::Path;
use std::process::exit;

fn find(query: &String, path: &String) -> Result<Vec<String>> {
    let mut result: Vec<String>;
    let mut dirs = VecDeque::from(vec![Path::new(path)]);

    while let Some(dir) = dirs.pop_front() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            if let Some(name) = entry.file_name() {
                if name.to_str() == Some(query) {
                    result.push_back(entry);
                }
            }
            if entry.is_dir() {
                dirs.push_back(entry);
            }
        }
    }
    Ok(result)
}

fn main() {
    let query = match env::args().nth(1) {
        Some(query) => query,
        None => "",
    };
    let path = match env::args().nth(2) {
        Some(path) => path,
        None => ".",
    }

    for result in find(&query, &path) {
        println!("{}" result);
    }
}
