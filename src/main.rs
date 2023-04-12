mod cli;
mod crypto;

use std::collections::HashMap;

use cli::stdio::{read_from_stdin, read_lines_from_file};
use crypto::{bytes_ext::BytesExt, str_ext::StrExt};

const HEX_TO_BYTES_TABLE: [(char, u8); 22] = [
    ('0', 0),
    ('1', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('a', 10),
    ('b', 11),
    ('c', 12),
    ('d', 13),
    ('e', 14),
    ('f', 15),
    ('A', 10),
    ('B', 11),
    ('C', 12),
    ('D', 13),
    ('E', 14),
    ('F', 15),
];

fn main() {
    //assert!(('ô' as u8).is_ascii());
    //tests();
    //repeating_xor_stdin();
}

fn hex_test() {
    (0..=u8::MAX)
}

fn repeating_xor_stdin() {
    println!(
        "{}",
        String::from_utf8(
            read_lines_from_file("/home/popuser/cryptopals/src/4.txt")
                .into_bytes()
                .xor(&read_from_stdin())
        )
        .unwrap()
    );
}

fn tests() {
    //Challenge 1
    assert_eq!(<[u8]>::from_hex(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").to_base64(),
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    //Challenge 2
    assert_eq!(
        <[u8]>::from_hex(&"1c0111001f010100061a024b53535009181c")
            .xor(&<[u8]>::from_hex(&"686974207468652062756c6c277320657965"))
            .to_hex(),
        "746865206b696420646f6e277420706c6179"
    );

    //Challenge 3
    let single_byte_xor_cipher =
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let chall_3_key = <[u8]>::from_hex(&single_byte_xor_cipher).bf_single_xor();
    println!(
        "Challenge 3's plaintext is \"{}\"",
        String::from_utf8(<[u8]>::from_hex(&single_byte_xor_cipher).single_xor(chall_3_key))
            .unwrap()
    );

    //Challenge 4

    let cipher = include_str!("4.txt")
        .lines()
        .min_by_key(|x| {
            let bytes = <[u8]>::from_hex(x);
            bytes
                .single_xor(bytes.bf_single_xor())
                .english_frequency_fitting_quotient()
        })
        .unwrap();
    let chall_4_key = <[u8]>::from_hex(cipher).bf_single_xor();
    println!(
        "Challenge 4's plaintext is \"{}\"",
        <[u8]>::from_hex(cipher).single_xor(chall_4_key).to_string()
    );

    //Challenge 5
    assert_eq!(
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string().into_bytes()
                .xor(&"ICE".as_bytes()).to_hex(),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );

    //Challenge 6
    let chall_6_plaintext = "this is a test";
    let chall_6_cipher = "wokka wokka!!!";
    assert_eq!(chall_6_plaintext.hamming_distance(&chall_6_cipher), 37);

    let test = <[u8]>::from_hex("1420203d2272313f212d22723c203e373d722b2626782e3f3d3b7e782c3d363c373b3b372c3a20782e36313f3b2b2c3b3628723d233b2c63722b2a36782b3d782a3b2d3c3f372b722c2a3f28202078263c3b2636312b27363b722d3b72342e30373d37782a26782b3d3420203d6f3f39283c396f333426232d2e7c781a26782a3c312272392b7235263c3122722e2a3c312e3f746f232d262178213d2b3b202d2b723d37372a2c3b2c2e2631203c783a3e342e3f3b2072342e30373d3b2b6f3c313c3b783a26782e3e313e27313f723d37723d2e723b203f352036376f313721213d3e27393b7c780b27313c72393a263d6f3b2a3a203d6f3637233d2a6f3b366f203d3f203d2737362b372a262678263c78393d343a222c2e263d6f243d233b2c6f372b3c37782c3b342327356f3637233d2a2a723d3a723e3a35312e26782127342333783f332a26332c3a20766f17202c37283b372d3d722b263c2c6f3d3b2c333d2c332c6f312d3f3b3c2e26393b7236203c783f203726363d2126746f212d212678263c782c27343f33783e27316f3d3e293b3b2633782b372b2a202d212678223d34233b2c6f3336263f782636782a212c6f3e392d3d2a3a3f76");
    println!(
        "Challenge 6's plaintext is \"{}\"",
        test.xor(&test.bf_repeating_xor()).to_string()
    );
}
