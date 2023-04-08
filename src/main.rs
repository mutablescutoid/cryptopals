mod cli;
mod crypto;

use crypto::bytes_ext::BytesExt;

fn main() {
    /*let stdin = stdio::read_from_stdin().iter().map(|x| *x as char).collect();
    match hex::hex_to_bytes(stdin) {
        Ok(bytes) => {
            println!("\n{}", base64::bytes_to_base64(bytes));
        }
        Err(err) => {
            println!("\n{}", err)
        }
    }
    let str_one = hex::hex_to_bytes(&String::from("1c0111001f010100061a024b53535009181c")).unwrap();
    let str_two = hex::hex_to_bytes(&String::from("686974207468652062756c6c277320657965")).unwrap();
    //println!("{}", hex::bytes_to_hex(&xor::fixed_xor(&str_one, &str_two).unwrap()));
    println!("{}", str_one.to_base64())*/

    assert_eq!(Vec::from_hex(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")).to_base64(),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
}
