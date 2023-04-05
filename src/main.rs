mod cli;
mod crypto;

use crypto::hex;
use crypto::base64::Base64;

fn main() {
    /*let stdin = stdio::read_from_stdin().iter().map(|x| *x as char).collect();
    match hex::hex_to_bytes(stdin) {
        Ok(bytes) => {
            println!("\n{}", base64::bytes_to_base64(bytes));
        }
        Err(err) => {
            println!("\n{}", err)
        }
    }*/
    let str_one = hex::hex_to_bytes(&String::from("1c0111001f010100061a024b53535009181c")).unwrap();
    let str_two = hex::hex_to_bytes(&String::from("686974207468652062756c6c277320657965")).unwrap();
    //println!("{}", hex::bytes_to_hex(&xor::fixed_xor(&str_one, &str_two).unwrap()));
    println!("{}", str_one.to_base64())
}
