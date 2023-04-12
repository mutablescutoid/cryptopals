use super::{
    BASE64_DIVISOR, BASE64_PADDING, BASE64_TABLE, BASE64_TO_BINARY_TABLE, BYTE_DIVISOR,
    ENGLISH_FREQUENCY_TABLE, HEX_TABLE, HEX_TO_BYTE_TABLE,
};
use std::collections::HashMap;

pub trait BytesExt {
    fn to_string(&self) -> String;

    fn to_base64(&self) -> String;

    fn from_base64(base64_str: &str) -> Vec<u8>;

    fn to_hex(&self) -> String;

    fn from_hex(hex_str: &str) -> Vec<u8>;

    fn xor(&self, key: &[u8]) -> Vec<u8>;

    fn single_xor(&self, key: u8) -> Vec<u8>;

    fn bf_single_xor(&self) -> u8;

    fn english_frequency_fitting_quotient(&self) -> i64;

    fn hamming_distance(&self, string: &[u8]) -> usize;

    fn bf_repeating_xor(&self) -> Vec<u8>;
}

impl BytesExt for [u8] {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.iter().map(|x| *x).collect::<Vec<u8>>()).to_string()
    }

    fn to_base64(&self) -> String {
        let mut base64_str = String::from("");

        let mut bytes_index = 0;
        let mut scratch_byte = 0x00;
        for byte in self {
            match bytes_index % BYTE_DIVISOR {
                0 => {
                    base64_str.push(BASE64_TABLE[(byte >> 2) as usize]); //Utilizes first six bits
                    scratch_byte = byte << 6 >> 2; //Places the remaining two bits at the beginning of the next six bits
                }
                1 => {
                    base64_str.push(BASE64_TABLE[((byte >> 4) + scratch_byte) as usize]); //Utilizes first byte's remaining two bits and the next four bits
                    scratch_byte = byte << 4 >> 2; //Places the remaining four bits at the beginning of next the next six bits
                }
                2 => {
                    base64_str.push(BASE64_TABLE[((byte >> 6) + scratch_byte) as usize]); //Utilizes second byte's remaining four bits and the next two bits
                    base64_str.push(BASE64_TABLE[(byte << 2 >> 2) as usize]); //Utilizes the last six bits
                    scratch_byte = 0x00;
                }
                _ => {}
            }
            bytes_index += 1;
        }

        if bytes_index % BYTE_DIVISOR != 0 {
            //If the scratch byte contains still relevant info
            base64_str.push(BASE64_TABLE[scratch_byte as usize]);
        }

        while base64_str.len() % BASE64_DIVISOR != 0 {
            base64_str.push(BASE64_PADDING);
        }

        base64_str
    }

    fn from_base64(base64_str: &str) -> Vec<u8> {
        //let base64_to_binary_map = HashMap::from(BASE64_TO_BINARY_TABLE);

        let mut bytes = Vec::new();
        let mut scratch_byte = 0x00;

        let mut base64_str_index = 0;
        for char in base64_str.chars() {
            let binary = if BASE64_TO_BINARY_TABLE[char as usize] != 0x00 {
                BASE64_TO_BINARY_TABLE[char as usize]
            } else {
                break;
            };

            match base64_str_index % BASE64_DIVISOR {
                0 => {
                    scratch_byte = binary << 2;
                }
                1 => {
                    bytes.push(scratch_byte + (binary >> 4));
                    scratch_byte = binary << 4;
                }
                2 => {
                    bytes.push(scratch_byte + (binary >> 2));
                    scratch_byte = binary << 6;
                }
                3 => {
                    bytes.push(scratch_byte + binary);
                    scratch_byte = 0x00;
                }
                _ => {}
            }

            base64_str_index += 1;
        }

        bytes
    }

    fn to_hex(&self) -> String {
        self.iter()
            .flat_map(|x| {
                //For each byte, maps an array of two hexdigits, then flattens them into a sequence them of hexdigits
                [
                    HEX_TABLE[(x >> 4) as usize] as u8,
                    HEX_TABLE[(x << 4 >> 4) as usize] as u8,
                ]
            })
            .collect::<Vec<u8>>()
            .to_string()
    }

    fn from_hex(hex_str: &str) -> Vec<u8> {
        hex_str
            .chars()
            .map(|x| HEX_TO_BYTE_TABLE[x as usize]) //Convert them into bytes
            .collect()
    }

    fn xor(&self, key: &[u8]) -> Vec<u8> {
        self.iter()
            .zip(key.iter().cycle())
            .map(|x| *x.0 ^ *x.1)
            .collect()
    }

    fn single_xor(&self, key: u8) -> Vec<u8> {
        self.xor(&key.to_ne_bytes())
    }

    fn bf_single_xor(&self) -> u8 {
        (0..=u8::MAX)
            .into_iter()
            .min_by_key(|x| {
                let score = self.single_xor(*x).english_frequency_fitting_quotient();
                println!(
                    "{} {} {}",
                    score,
                    *x as char,
                    self.single_xor(*x).to_string()
                );
                score
            })
            .unwrap()
    }

    //Currently returns the fitting quotient * i64::MAX
    fn english_frequency_fitting_quotient(&self) -> i64 {
        if !self.iter().all(|x| {
            (*x).is_ascii_alphabetic() || (*x).is_ascii_punctuation() || (*x).is_ascii_whitespace()
        }) {
            return i64::MAX;
        }

        let english_frequency_map = HashMap::from(ENGLISH_FREQUENCY_TABLE);

        english_frequency_map.keys().into_iter().fold(0, |acc, x| {
            acc + ((english_frequency_map.get(x).unwrap()
                - self.iter().fold(0, |acc, y| {
                    acc + if *x == y.to_ascii_lowercase() as char {
                        1
                    } else {
                        0
                    }
                }) * (i64::MAX / self.len() as i64))
                / self.len() as i64)
                .abs()
        }) / english_frequency_map.len() as i64
    }

    fn hamming_distance(&self, other: &[u8]) -> usize {
        self.iter()
            .zip(other)
            .map(|x| (x.0 ^ x.1).count_ones() as usize)
            .sum()
    }

    fn bf_repeating_xor(&self) -> Vec<u8> {
        let key_size = (2..=40)
            .into_iter()
            .min_by_key(|x| match (self.get(0..*x), self.get(*x..2 * *x)) {
                (Some(first_key_size_bytes), Some(second_key_size_bytes)) => {
                    first_key_size_bytes.hamming_distance(second_key_size_bytes) / x
                }
                _ => usize::MAX,
            })
            .unwrap();

        (0..key_size)
            .into_iter()
            .map(|x| {
                let block = self
                    .into_iter()
                    .enumerate()
                    .skip(x)
                    .step_by(key_size)
                    .map(|y| {
                        println!("{}", y.0);
                        *y.1
                    })
                    .collect::<Vec<u8>>();

                let key = block.bf_single_xor();

                println!(
                    "key = \'{}\' plaintext= \"{}\" block = \"{}\"",
                    key as char,
                    String::from_utf8(block.single_xor(key)).unwrap(),
                    String::from_utf8(block).unwrap()
                );

                key
            })
            .collect::<Vec<u8>>()
    }
}
