extern crate libc;
extern crate termios;
#[macro_use(defer)]
extern crate scopeguard;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;

use termios::*;

fn pagefile(filename: &str, pagesize: u16) -> io::Result<()> {
    let file = File::open(filename)?;

    let mut buf = BufReader::new(file);
    let mut eof = false;
    while !eof {
        for _ in 0..pagesize {
            let mut line = String::new();
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

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("Missing file arg");

    // C library calls
    let pagesize: u16;
    let mut save: libc::termios;

    let mut term = Termios::from_fd(libc::STDIN_FILENO)?;
    tcgetattr(&term);
    let mut save = Termios {..term};
    term.c_lflag &= !(ICANON | ECHO);
    tcset

    unsafe {
        save = mem::zeroed();
        libc::tcgetattr(, &mut save);

        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);

        let winsz: libc::winsize = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &winsz);
        pagesize = winsz.ws_row;
    }
    defer!(unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &save); });

    pagefile(&filename, pagesize)?;

    Ok(())
}
