extern crate libc;

use std::env;
use std::mem;
use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::process::exit;

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Missing file arg");
            exit(1);
        }
    }

    // C library calls
    unsafe {
        let mut save: libc::termios = mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut save);

        let mut term = libc::termios {..save};
        term.c_lflag &= !(libc::ICANON | libc::ECHO);
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);

        let winsz: libc::winsize = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &winsz);
    }
    let pagesize = winsz.ws_row;

    let file = File::open(filename).expect("Unable to open file");

    let mut lines = BufReader::new(file).lines().peekable();
    while let Some(_) = lines.peek() {
        for _ in 0..pagesize {
            match lines.next() {
                Some(line) => {
                    match line {
                        Ok(l) => println!("{}", l),
                        Err(e) => exit(1);
                    }
                    println!("{}", line.unwrap());
                },
                None => {
                    break;
                }
            }
        }
        let mut c = [0;1];
        std::io::stdin().read_exact(&mut c).unwrap();
    }

    unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &save); }
}
