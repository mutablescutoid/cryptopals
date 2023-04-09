use super::{
    ARGUMENT_SIZES_DIFFER, BASE64_DIVISOR, BASE64_PADDING, BASE64_TABLE, BASE64_TO_BINARY_TABLE,
    BYTE_DIVISOR, HEX_TABLE, HEX_TO_BYTES_TABLE,
};
use std::collections::HashMap;

pub trait BytesExt {
    fn to_base64(&self) -> String;

    fn from_base64(base64_str: &String) -> Vec<u8>;

    fn to_hex(&self) -> String;

    fn from_hex(hex_str: &String) -> Vec<u8>;

    fn fixed_xor(&self, key: &Vec<u8>) -> Result<Vec<u8>, &'static str>;

    fn single_byte_xor(&self, key: u8) -> Vec<u8>;

    fn find_single_byte_key(&self) -> u8;

    fn english_frequency_fitting_quotient(&self) -> f64;

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

    fn from_base64(base64_str: &String) -> Vec<u8> {
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

    fn from_hex(hex_str: &String) -> Vec<u8> {
        let hex_to_bytes_map = HashMap::from(HEX_TO_BYTES_TABLE);

        let mut bytes = Vec::new();

        let mut scratch_byte_modified = false;
        let mut scratch_byte = 0x00;

        for char in hex_str.chars() {
            match hex_to_bytes_map.get(&char) {
                Some(value) => {
                    if !scratch_byte_modified {
                        scratch_byte_modified = true;
                        scratch_byte = 0x10 * value;
                    } else {
                        bytes.push(scratch_byte + value);
                        scratch_byte = 0x00;
                        scratch_byte_modified = false;
                    }
                }
                None => {}
            }
        }

        bytes
    }

    fn to_hex(&self) -> String {
        let mut hex_str = String::from("");

        for byte in self {
            hex_str.push(HEX_TABLE[(byte >> 4) as usize]);
            hex_str.push(HEX_TABLE[(byte << 4 >> 4) as usize]);
        }

        hex_str
    }

    fn fixed_xor(&self, key: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
        if key.len() != self.len() {
            return Err(ARGUMENT_SIZES_DIFFER);
        }

        let mut key_iter = key.iter();
        Ok(self.iter().map(|x| x ^ key_iter.next().unwrap()).collect())
    }

    fn single_byte_xor(&self, key: u8) -> Vec<u8> {
        self.iter().map(|x| x ^ key).collect()
    }

    fn find_single_byte_key(&self) -> u8 {
        let mut max_score: f64 = f64::MAX;
        let mut max_key = 0x00;
        for key in 0..=u8::MAX {
            let score = self
                .single_byte_xor(key)
                .english_frequency_fitting_quotient();         

            if score < max_score {
                max_score = score;
                max_key = key;
            }
        }

        max_key
    }

    fn english_frequency_fitting_quotient(&self) -> f64 {
        let english_frequency_map = HashMap::from([
            ('a', 8.2389258),
            ('b', 1.5051398),
            ('c', 2.8065007),
            ('d', 4.2904556),
            ('e', 12.813865),
            ('f', 2.2476217),
            ('g', 2.0327458),
            ('h', 6.1476691),
            ('i', 6.1476691),
            ('j', 0.1543474),
            ('k', 0.7787989),
            ('l', 4.0604477),
            ('m', 2.4271893),
            ('n', 6.8084376),
            ('o', 7.5731132),
            ('p', 1.9459884),
            ('q', 0.0958366),
            ('r', 6.0397268),
            ('s', 6.3827211),
            ('t', 9.1357551),
            ('u', 2.7822893),
            ('v', 0.9866131),
            ('w', 2.3807842),
            ('x', 0.1513210),
            ('y', 1.9913847),
            ('z', 0.0746517),
        ]);

        english_frequency_map
            .keys()
            .into_iter() //For every letter in the alphabet
            .fold(0.0, |acc, x| {
                //Add
                acc + (english_frequency_map.get(x).unwrap() //The relative frequency in the English language
                    - self.iter().fold(0.0, |acc, y| { //Minus the relative frequency in the text
                        acc + if *x == y.to_ascii_lowercase() as char {
                            1
                        } else {
                            0
                        } as f64
                    }) / self.len() as f64)
                    .abs() //As an absolute value
            })
            / english_frequency_map.len() as f64 //And average it out
    }

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8> {
        let mut key_iter = key.iter().cycle();

        self.iter().map(|x| *x ^ key_iter.next().unwrap()).collect()
    }

    fn break_repeating_key_xor(&self) -> Vec<u8> {
        todo!();
    }
}
