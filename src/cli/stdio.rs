use std::{fs::File, io, io::Read};

const LF: u8 = 0x0A; //Line feed, or Linux's newline character

pub fn read_from_stdin() -> Vec<u8> {
    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin).unwrap();

    let last_byte = stdin[stdin.len() - 1];
    if last_byte == LF {
        stdin.pop();
    }

    stdin
}

pub fn read_lines_from_file(path: &str) -> String {
    let mut lines = String::new();
    let _ = File::open(path).unwrap().read_to_string(&mut lines);
    lines
}
