use super::{
    ARGUMENT_SIZES_DIFFER, BASE64_DIVISOR, BASE64_PADDING, BASE64_TABLE, BASE64_TO_BINARY_TABLE,
    BYTE_DIVISOR, HEX_TABLE, HEX_TO_BYTES_TABLE, HIGHEST_ASCII, LOWEST_ASCII,
};
use std::collections::HashMap;

pub trait BytesExt {
    fn to_base64(self) -> String;

    fn from_base64(base64_str: String) -> Vec<u8>;

    fn to_hex(self) -> String;

    fn from_hex(hex_str: String) -> Vec<u8>;

    fn fixed_xor(&self, key: &Vec<u8>) -> Result<Vec<u8>, &'static str>;

    fn single_byte_xor(&self, key: u8) -> Vec<u8>;

    fn find_single_byte_key(&self) -> u8;

    fn plaintext_score(&self) -> usize;

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8>;
}

impl BytesExt for Vec<u8> {
    fn to_base64(self) -> String {
        let mut base64_str = String::from("");

        //self.iter().enumerate();

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

    fn from_base64(base64_str: String) -> Vec<u8> {
        let base64_to_binary_map = HashMap::from(BASE64_TO_BINARY_TABLE);

        let bytes = Vec::new();
        for base64_byte in base64_str.clone().into_bytes() {}
        bytes
    }

    fn from_hex(hex_str: String) -> Vec<u8> {
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

    fn to_hex(self) -> String {
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

        /*         let mut ciphertext = Vec::with_capacity(self.len());

        let mut plaintext_index = 0;
        for key_byte in key {
            let plaintext_byte = self.get(plaintext_index).unwrap();
            ciphertext.push(key_byte ^ plaintext_byte);

            plaintext_index += 1;
        }

        Ok(ciphertext)*/
        let mut key_iter = key.iter();
        Ok(self
            .clone()
            .into_iter()
            .map(|x| x ^ key_iter.next().unwrap())
            .collect())
    }

    fn single_byte_xor(&self, key: u8) -> Vec<u8> {
        /*let mut ciphertext = Vec::with_capacity(self.len());

        for plaintext_byte in self {
            ciphertext.push(key ^ plaintext_byte);
        }

        ciphertext*/

        self.clone().into_iter().map(|x| x ^ key).collect()
    }

    fn find_single_byte_key(&self) -> u8 {
        let mut max_score = 0;
        let mut max_key = 0;
        for key in 0..=u8::MAX {
            let score = Self::plaintext_score(&self.single_byte_xor(key));
            if score > max_score {
                max_score = score;
                max_key = key;
            }
        }

        max_key
    }

    fn plaintext_score(&self) -> usize {
        /*let mut count = 0;

        for plaintext_byte in self {
            if *plaintext_byte >= Self::LOWEST_ASCII as u8
                && *plaintext_byte <= Self::HIGHEST_ASCII as u8
            {
                count += 1;
            }
        }

        count*/

        self.clone().iter().fold(0, |acc, x| {
            if *x >= LOWEST_ASCII as u8 && *x <= HIGHEST_ASCII as u8 {
                1
            } else {
                0
            }
        })
    }

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8> {
        /*       let mut ciphertext = plaintext.clone();

        for key_byte in key {
            for n in 0..ciphertext.len() - 1 {
                ciphertext[n] = key_byte ^ ciphertext[n];
            }
        }

        ciphertext*/

        self.clone()
            .into_iter()
            .map(|x| key.clone().iter().fold(x, |x, y| y ^ x))
            .collect()
    }
}
