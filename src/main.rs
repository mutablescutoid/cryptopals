mod cli;
mod crypto;

use crypto::{bytes_ext::BytesExt, str_ext::StrExt};
use std::{fs::File, io::Read};

fn main() {
    //Challenge 1
    assert_eq!(Vec::from_hex(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").to_base64(),
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    //Challenge 2
    assert_eq!(
        Vec::from_hex(&"1c0111001f010100061a024b53535009181c")
            .fixed_xor(&Vec::from_hex(&"686974207468652062756c6c277320657965"))
            .to_hex(),
        "746865206b696420646f6e277420706c6179"
    );

    //Challenge 3
    let single_byte_xor_cipher =
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let chall_3_key = Vec::from_hex(&single_byte_xor_cipher).find_single_byte_key();
    println!(
        "Challenge 3's plaintext is \"{}\"",
        String::from_utf8(Vec::from_hex(&single_byte_xor_cipher).single_byte_xor(chall_3_key))
            .unwrap()
    );

    //Challenge 4
    let mut lines = String::new();
    let _chall_4_txt = File::open("4.txt").unwrap().read_to_string(&mut lines);
    let cipher = lines
        .lines()
        .min_by_key(|x| {
            let bytes = Vec::from_hex(x);
            bytes
                .single_byte_xor(bytes.find_single_byte_key())
                .english_frequency_fitting_quotient()
        })
        .unwrap();
    let chall_4_key = Vec::from_hex(cipher).find_single_byte_key();
    println!(
        "Challenge 4's plaintext is \"{}\"",
        String::from_utf8(Vec::from_hex(cipher).single_byte_xor(chall_4_key)).unwrap()
    );

    //Challenge 5
    assert_eq!(
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string().into_bytes()
                .repeating_key_xor(&"ICE".to_string().into_bytes()).to_hex(),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );

    //Challenge 6
    assert_eq!("this is a test".hamming_distance(&"wokka wokka!!!"), 37);
}
