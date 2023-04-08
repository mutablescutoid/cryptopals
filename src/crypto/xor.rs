use std::collections::HashMap;

pub trait Xor {
    const ARGUMENT_SIZES_DIFFER: &'static str = "Size of arguments differ";
    const LOWEST_ASCII: char = ' ';
    const HIGHEST_ASCII: char = '~';

    fn fixed_xor(&self, key: &Vec<u8>) -> Result<Vec<u8>, &'static str>;

    fn single_byte_xor(&self, key: u8) -> Vec<u8>;

    fn find_single_byte_key(&self) -> u8;

    fn plaintext_score(&self) -> usize;

    fn repeating_key_xor(&self, key: &Vec<u8>) -> Vec<u8>;
}

impl Xor for Vec<u8> {
    fn fixed_xor(&self, key: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
        if key.len() != self.len() {
            return Err(Self::ARGUMENT_SIZES_DIFFER);
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

        self.clone.fold(0, |acc, x| if *x >= Self::LOWEST_ASCII as u8
                && *x <= Self::HIGHEST_ASCII as u8 {1} else {0})
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
            .map(|x| {
                key.clone().fold(|y| y ^ x)
            })
            .collect()
    }
}
