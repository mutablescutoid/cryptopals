use super::{BASE64_PADDING, BASE64_TABLE, HEX_TABLE};
use std::collections::HashSet;
pub trait StringExt {
    fn is_base64(&self) -> bool;

    fn is_hex(&self) -> bool;

    fn hamming_distance(&self, string: &String) -> usize;
}

impl StringExt for String {
    fn is_base64(&self) -> bool {
        self.chars()
            .all(|x| HashSet::from(BASE64_TABLE).contains(&x) || x == BASE64_PADDING)
    }

    fn is_hex(&self) -> bool {
        self.chars().all(|x| HashSet::from(HEX_TABLE).contains(&x)) && self.len() % 2 == 0
    }

    fn hamming_distance(&self, string: &String) -> usize {
        let mut string_iter = string.as_bytes().iter();

        self.as_bytes()
            .iter()
            .map(|x| {
                let xor = x ^ string_iter.next().unwrap();
                (0..=7)
                    .into_iter()
                    .fold(0, |acc, y| acc + (xor << y >> 7) as usize)
            })
            .sum()
    }
}
