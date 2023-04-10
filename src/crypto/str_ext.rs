use super::{BASE64_PADDING, BASE64_TABLE, HEX_TABLE};

pub trait StrExt {
    fn is_base64(&self) -> bool;

    fn is_hex(&self) -> bool;

    fn hamming_distance(&self, string: &str) -> usize;
}

impl StrExt for str {
    fn is_base64(&self) -> bool {
        self.chars()
            .all(|x| BASE64_TABLE.contains(&x) || x == BASE64_PADDING)
    }

    fn is_hex(&self) -> bool {
        self.chars().all(|x| HEX_TABLE.contains(&x)) && self.len() % 2 == 0
    }

    fn hamming_distance(&self, string: &str) -> usize {
        self.as_bytes()
            .iter()
            .zip(string.as_bytes())
            .map(|x| (x.0 ^ x.1).count_ones() as usize)
            .sum()
    }
}
