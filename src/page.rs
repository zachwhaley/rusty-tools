extern crate libc;

use std::env;
use std::mem;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let filename = env::args().nth(1).expect("Missing file arg");

    let mut save: libc::termios = unsafe { mem::zeroed() };
    unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut save); }
    let mut term = libc::termios {..save};
    term.c_lflag &= !libc::ECHO;
    unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term); }

    let winsz: libc::winsize = unsafe { mem::zeroed() };
    unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &winsz); }
    let pagesize = winsz.ws_row;

    let file = File::open(filename).expect("Unable to open file");
    let mut lines = BufReader::new(file).lines().peekable();
    while let Some(_) = lines.peek() {
        for _ in 0..pagesize {
            match lines.next() {
                Some(line) => {
                    println!("{}", line.unwrap());
                },
                None => {
                    break;
                }
            }
        }
    }

    unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &save); }
}
