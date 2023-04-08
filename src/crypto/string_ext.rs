use super::{BASE64_PADDING, BASE64_TABLE, HEX_TABLE};
use std::collections::HashSet;
pub trait StringExt {
    fn is_base64(&self) -> bool;

    fn is_hex(&self) -> bool;

    fn hamming_distance(&self, string: &Vec<u8>) -> usize;
}

impl StringExt for String {
    fn is_base64(&self) -> bool {
        self.chars()
            .all(|x| HashSet::from(BASE64_TABLE).contains(&x) || x == BASE64_PADDING)
    }

    fn is_hex(&self) -> bool {
        self.chars().all(|x| HashSet::from(HEX_TABLE).contains(&x)) && self.len() % 2 == 0
    }

    fn hamming_distance(&self, string: &Vec<u8>) -> usize {
        0
    }
}
