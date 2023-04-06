use std::io;
use std::io::Read;

const LF: u8 = 0x0A; //Line feed, or Linux's newline character

fn read_from_stdin() -> Vec<u8> {
    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin).unwrap();

    let last_byte = stdin[stdin.len() - 1];
    if last_byte == LF {
        stdin.pop();
    }

    stdin
}
