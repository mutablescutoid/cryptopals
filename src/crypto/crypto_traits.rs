//pub fn hamming_distance(str_one: Vec<u8>, str_two: Vec<u8>) -> usize {

//}

pub trait StringManipulation {
    fn hamming_distance(&self, bytes: &Vec<u8>) -> usize;

    fn from_bytes() -> String;
}