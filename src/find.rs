use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fs::{self, ReadDir, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

struct Find<'a> {
    query: &'a OsStr,
    readdir: ReadDir,
    dirs: VecDeque<PathBuf>,
}
impl<'a> Find<'a> {
    fn new(query: &'a OsStr, start: &'a OsStr) -> io::Result<Find<'a>> {
        Ok(Find {
            query: query,
            readdir: fs::read_dir(Path::new(start))?,
            dirs: VecDeque::new(),
        })
    }
}
impl<'a> Iterator for Find<'a> {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        let m = {
            let dirs = &mut self.dirs;
            let query = &self.query;
            self.readdir.find(|entry| {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            dirs.push_back(path.clone());
                        }
                        if let Some(name) = path.file_name() {
                            *query == name
                        } else {
                            false
                        }
                    },
                    Err(err) => {
                        eprintln!("{}", err);
                        false
                    }
                }
            })
        };
        if let None = m {
            if let Some(dir) = self.dirs.pop_front() {
                self.readdir = fs::read_dir(dir).unwrap();
                return self.next();
            }
        }
        return m;
    }
}

fn find<'a>(query: &'a OsStr, start: &'a OsStr) -> io::Result<Find<'a>> {
    Find::new(query, start)
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let query = args.next().map(OsString::from).unwrap();
    let start = args.next().map(OsString::from).unwrap_or(OsString::from("."));

    for entry in find(&query, &start)? {
        if let Some(result) = entry?.path().to_str() {
            println!("{}", result);
        }
    }
    Ok(())
}
