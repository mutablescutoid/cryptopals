use super::{
    BASE64_DIVISOR, BASE64_PADDING, BASE64_TABLE, BASE64_TO_BINARY_TABLE, BYTE_DIVISOR,
    ENGLISH_FREQUENCY_TABLE, HEX_TABLE, HEX_TO_BYTES_TABLE,
};
use std::collections::HashMap;

pub trait BytesExt {
    fn to_base64(&self) -> String;

    fn from_base64(base64_str: &str) -> Vec<u8>;

    fn to_hex(&self) -> String;

    fn from_hex(hex_str: &str) -> Vec<u8>;

    fn fixed_xor(&self, key: &Vec<u8>) -> Vec<u8>;

    fn single_byte_xor(&self, key: u8) -> Vec<u8>;

    fn find_single_byte_key(&self) -> u8;

    fn english_frequency_fitting_quotient(&self) -> i64;

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8>;

    fn break_repeating_key_xor(&self) -> Vec<u8>;
}

impl BytesExt for Vec<u8> {
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
        let base64_to_binary_map = HashMap::from(BASE64_TO_BINARY_TABLE);

        let mut bytes = Vec::new();
        let mut scratch_byte = 0x00;

        let mut base64_str_index = 0;
        for char in base64_str.chars() {
            let binary;
            match base64_to_binary_map.get(&char) {
                Some(value) => {
                    binary = *value;
                }
                None => {
                    break;
                }
            }

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

    fn from_hex(hex_str: &str) -> Vec<u8> {
        hex_str
            .chars()
            .zip(hex_str.chars().skip(1)) //Take two consecutive, overlapping chars at a time
            .step_by(2) //Skip such that they touch but don't overlap
            .map(|x| {
                HashMap::from(HEX_TO_BYTES_TABLE).get(&x.0).unwrap() * 0x10
                    + HashMap::from(HEX_TO_BYTES_TABLE).get(&x.1).unwrap()
            }) //Convert them into bytes
            .collect()
    }

    fn to_hex(&self) -> String {
        String::from_utf8(
            self.iter()
                .flat_map(|x| {
                    [
                        HEX_TABLE[(x >> 4) as usize] as u8,
                        HEX_TABLE[(x << 4 >> 4) as usize] as u8,
                    ]
                })
                .collect(),
        )
        .unwrap()
    }

    fn fixed_xor(&self, key: &Vec<u8>) -> Vec<u8> {
        self.into_iter().zip(key).map(|x| *x.0 ^ *x.1).collect()
    }

    fn single_byte_xor(&self, key: u8) -> Vec<u8> {
        self.iter().map(|x| x ^ key).collect()
    }

    fn find_single_byte_key(&self) -> u8 {
        (0..=u8::MAX)
            .into_iter()
            .min_by_key(|x| {
                self.single_byte_xor(*x)
                    .english_frequency_fitting_quotient()
            })
            .unwrap()
    }

    //Currently returns the fitting quotient * i64::MAX
    fn english_frequency_fitting_quotient(&self) -> i64 {
        if !self.is_ascii() || self.iter().any(|x| *x == 0x00) {
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

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8> {
        self.iter()
            .zip(key.iter().cycle())
            .map(|x| *x.0 ^ *x.1)
            .collect()
    }

    fn break_repeating_key_xor(&self) -> Vec<u8> {
        todo!();
    }
}
