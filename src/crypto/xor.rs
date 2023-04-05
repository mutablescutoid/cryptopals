use std::collections::HashMap;

const ARGUMENT_SIZES_DIFFER: &str = "Size of arguments differ";
const LOWEST_ASCII: char = ' ';
const HIGHEST_ASCII: char = '~';

pub fn fixed_xor(key: &Vec<u8>, plaintext: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    if key.len() != plaintext.len() {
        return Err(ARGUMENT_SIZES_DIFFER);
    }

    let mut ciphertext = Vec::with_capacity(plaintext.len());

    let mut plaintext_index = 0;
    for key_byte in key {
        let plaintext_byte = plaintext.get(plaintext_index).unwrap();
        ciphertext.push(key_byte ^ plaintext_byte);

        plaintext_index += 1;
    }

    Ok(ciphertext)
}

pub fn single_byte_xor(key: u8, plaintext: &Vec<u8>) -> Vec<u8> {
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for plaintext_byte in plaintext {
        ciphertext.push(key ^ plaintext_byte);
    }

    ciphertext
}

pub fn find_single_byte_key(ciphertext: &Vec<u8>) -> u8 {
    let mut weighted_keys = HashMap::new();

    let mut max_score = 0;
    let mut max_key = 0;
    for key in 0..=u8::MAX {
        let score = plaintext_score(&single_byte_xor(key, ciphertext));
        weighted_keys.insert(key, score);
        if score > max_score {
            max_score = score;
            max_key = key;
        }
    }

    max_key
}

fn plaintext_score(plaintext: &Vec<u8>) -> usize {
    let mut count = 0;

    for plaintext_byte in plaintext {
        if *plaintext_byte >= LOWEST_ASCII as u8 && *plaintext_byte <= HIGHEST_ASCII as u8 {
            count += 1;
        }
    }

    count
}

pub fn repeating_key_xor(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u8> {
    let mut ciphertext = plaintext.clone();

    for key_byte in key {
        for n in 0..ciphertext.len() - 1 {
            ciphertext[n] = key_byte ^ ciphertext[n];
        }
    }

    //plaintext.clone().iter().map(|x| {for key_byte in key {*x ^= key_byte}});

    ciphertext
}
