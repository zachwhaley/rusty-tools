extern crate libc;
extern crate termios;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;

use termios::*;

fn window_size() -> io::Result<u16> {
    let winsz: libc::winsize;
    let rc = unsafe {
        winsz = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &winsz)
    };
    match rc {
        0 => Ok(winsz.ws_row),
        _ => Err(io::Error::last_os_error()),
    }
}

fn pagefile(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let pagesize = window_size()?;

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

    let mut term = Termios::from_fd(libc::STDIN_FILENO)?;
    let mut save = term.clone();
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(libc::STDIN_FILENO, TCSANOW, &mut term)?;

    if let Err(err) = pagefile(&filename) {
        // Always reset the terminal
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &mut save)?;
        return Err(err)
    }

    tcsetattr(libc::STDIN_FILENO, TCSANOW, &mut save)?;
    Ok(())
}
