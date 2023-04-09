mod cli;
mod crypto;

use crypto::{bytes_ext::BytesExt, string_ext::StringExt};
use std::{fs::File, io::Read};

fn main() {
    //Challenge 1
    assert_eq!(Vec::from_hex(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")).to_base64(),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));

    //Challenge 2
    assert_eq!(
        Vec::from_hex(&String::from("1c0111001f010100061a024b53535009181c"))
            .fixed_xor(&Vec::from_hex(&String::from(
                "686974207468652062756c6c277320657965"
            )))
            .unwrap()
            .to_hex(),
        String::from("746865206b696420646f6e277420706c6179")
    );

    //Challenge 3
    let single_byte_xor_cipher =
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
        //"332f2267342224282923672128352a672e346733302867223f37352234342e2829346734223726352633222367253e67266734222a2e24282b2829676f7c6e672229242b28342223672e296734363226352267253526242c2233346967132f2267223f37352234342e28296725222128352267332f22".to_string();
    let keys = Vec::from_hex(&single_byte_xor_cipher).find_single_byte_key();
    println!(
        "Challenge 3's plaintext is \"{}\"",
        String::from_utf8(Vec::from_hex(&single_byte_xor_cipher).single_byte_xor(keys)).unwrap()
    );

    //Challenge 4
    let mut lines = String::new();
    let mut chall4txt = File::open("4.txt").unwrap();
    chall4txt.read_to_string(&mut lines);
    let mut min_score = f64::MAX;
    let mut min_key = 0;
    let mut min_line = Vec::new();

    for line in lines.lines() {
        let bytes = Vec::from_hex(&line.to_string());
        let key = bytes.find_single_byte_key();
        let score = bytes
            .single_byte_xor(key)
            .english_frequency_fitting_quotient();

        match String::from_utf8(bytes.clone().single_byte_xor(key)) {
            Ok(value) => {
                println!(
                    "{} {} {} {}",
                    score,
                    key,
                    value,
                    line
                );
            }
            Err(err) => {}
        }
        

        if score < min_score {
            min_score = score;
            min_key = key;
            min_line = bytes.single_byte_xor(key);
        }
    }

    println!(
        "{} {} {}",
        min_score,
        min_key,
        String::from_utf8(min_line).unwrap()
    );

    //Challenge 5
    assert_eq!(
            String::from(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            )
            .into_bytes()
            .repeating_key_xor(&String::from("ICE").into_bytes()).to_hex(),
        String::from(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    );

    //Challenge 6
    assert_eq!(
        String::from("this is a test").hamming_distance(&String::from("wokka wokka!!!")),
        37
    );
}
