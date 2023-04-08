use std::collections::HashMap;

pub trait Hex {
    const HEX_TO_BYTES_TABLE: [(char, u8); 22] = [
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('a', 10),
        ('b', 11),
        ('c', 12),
        ('d', 13),
        ('e', 14),
        ('f', 15),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
    ];

    const BYTES_TO_HEX_TABLE: [(u8, char); 16] = [
        (0x00, '0'),
        (0x01, '1'),
        (0x02, '2'),
        (0x03, '3'),
        (0x04, '4'),
        (0x05, '5'),
        (0x06, '6'),
        (0x07, '7'),
        (0x08, '8'),
        (0x09, '9'),
        (0x0A, 'a'),
        (0x0B, 'b'),
        (0x0C, 'c'),
        (0x0D, 'd'),
        (0x0E, 'e'),
        (0x0F, 'f'),
    ];

    const MODIFIED: bool = true;
    const UNMODIFIED: bool = false;

    const WRONG_DIGIT_AMOUNT: &'static str = "Wrong amount of digits for a hexadecimal string";
    const NON_HEXADECIMAL_DIGIT: &'static str = "Non-hexadecimal digit found";

    fn to_hex(&self) -> String;
  
    fn from_hex(&self) -> Result<Vec<u8>, &'static str>;

    fn is_hex(&self) -> bool;
}

impl Hex for Vec<u8> {
    fn to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let hex_to_bytes_map = HashMap::from(Self::HEX_TO_BYTES_TABLE);

        if self.len() % 2 != 0 {
            return Err(Self::WRONG_DIGIT_AMOUNT);
        }
        let mut bytes: Vec<u8> = Vec::new();

        let mut scratch_byte = (0x00, Self::UNMODIFIED);

        for byte in self {
            match hex_to_bytes_map.get(&(*byte as char)) {
                Some(value) => {
                    if !scratch_byte.1 {
                        scratch_byte = (0x10 * value, Self::MODIFIED);
                    } else {
                        bytes.push(scratch_byte.0 + value);
                        scratch_byte = (0x00, Self::UNMODIFIED);
                    }
                }
                None => return Err(Self::NON_HEXADECIMAL_DIGIT),
            }
        }

        Ok(bytes)
    }

    fn to_hex(&self) -> String {
        let bytes_to_hex_map = HashMap::from(Self::BYTES_TO_HEX_TABLE);
        let mut hex_str = String::from("");

        for byte in self {
            let first_half = byte >> 4;
            let second_half = byte << 4 >> 4;
            match bytes_to_hex_map.get(&first_half) {
                Some(value) => {
                    hex_str.push(*value);
                }
                None => {}
            }
            match bytes_to_hex_map.get(&second_half) {
                Some(value) => {
                    hex_str.push(*value);
                }
                None => {}
            }
        }

        hex_str
    }
}
