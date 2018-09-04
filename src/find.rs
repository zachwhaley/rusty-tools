use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fs::{self, ReadDir, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

struct Find<'a> {
    query: &'a OsStr,
    start: &'a OsStr,
    readdir: io::Result<ReadDir>,
    dirs: VecDeque<PathBuf>,
}
impl<'a> Find<'a> {
    fn new(query: &'a OsStr, start: &'a OsStr) -> Find<'a> {
        Find {
            query: query,
            start: start,
            readdir: fs::read_dir(Path::new(start)),
            dirs: VecDeque::new(),
        }
    }
}
impl<'a> Iterator for Find<'a> {
    type Item = io::Result<DirEntry>
    fn next(&mut self) -> Option<Self::Item> {
        let dirs = &self.dirs;
        let query = &self.query;
        let mut readdir = self.readdir.unwrap();

        readdir.find(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push_back(path.clone());
                }
                if let Some(name) = path.file_name() {
                    return *query == name;
                }
            }
            return false;
        })
    }
}

fn find<'a>(query: &'a OsStr, start: &'a OsStr) -> Find<'a> {
    Find::new(query, start)
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let query = args.next().map(OsString::from).unwrap();
    let start = args.next().map(OsString::from).unwrap_or(OsString::from("."));

    for entry in find(&query, &start) {
        if let Some(result) = entry?.path().to_str() {
            println!("{}", result);
        }
    }
    Ok(())
}
