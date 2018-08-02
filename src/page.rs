extern crate libc;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;
use std::process::exit;

fn pagefile(filename: &String, pagesize: u16) -> io::Result<()> {
    let file = File::open(filename)?;

    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut eof = false;
    while !eof {
        for _ in 0..pagesize {
            match buf.read_line(&mut line)? {
                0 => {
                    eof = true;
                    break;
                },
                _ => println!("{}", line),
            }
        }
        let mut c = [0;1];
        std::io::stdin().read_exact(&mut c)?;
    }
    Ok(())
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            eprintln!("Missing file arg");
            exit(1);
        },
    };

    // C library calls
    let pagesize: u16;
    let mut save: libc::termios;
    unsafe {
        save = mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut save);

        let mut term = libc::termios {..save};
        term.c_lflag &= !(libc::ICANON | libc::ECHO);
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);

        let winsz: libc::winsize = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &winsz);
        pagesize = winsz.ws_row;
    }

    let rc = match pagefile(&filename, pagesize) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {}", err);
            1
        },
    };

    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &save);
    }

    exit(rc);
}
