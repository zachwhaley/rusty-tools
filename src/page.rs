extern crate libc;

use std::env;
use std::fs;
use std::mem;

fn main() {
    let filename = env::args().nth(1).expect("Missing file");
    let contents = fs::read_to_string(filename).expect("File read error");

    let mut save: libc::termios = unsafe { mem::zeroed() };
    unsafe {
        libc::tcgetattr(libc::STDIN_FILENO, &mut save);
    }

    let mut term = libc::termios {..save};
    term.c_lflag &= !libc::ECHO;
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);
    }

    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &save);
    }
}
