use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fs::{self, ReadDir, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

struct Find<'a> {
    query: &'a str,
    start: &'a OsStr,
    dirs: VecDeque<PathBuf>,
    entries: ReadDir,
}
impl<'a> Find<'a> {
    fn new(query: &'a str, start: &'a OsStr) -> io::Result<Find<'a>> {
        let f = Find {
            query: query,
            start: start,
            dirs: VecDeque::new(),
            entries: fs::read_dir(Path::new(start))?,
        };
        Ok(f)
    }
}
impl<'a> Iterator for Find<'a> {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        let it = entries.filter(|res| {
            match res {
                Ok(entry) => {
                    let path = entry.path();
                    if let Some(name) = path.file_name() {
                        if path.is_dir() {
                            self.dirs.push_back(path);
                        }
                        return self.query == name;
                    }
                },
                Err(err) => false
            }
        });
    }
}

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
